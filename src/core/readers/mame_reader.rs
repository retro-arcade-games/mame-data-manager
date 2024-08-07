use crate::core::data::MACHINES;
use crate::core::models::{BiosSet, CustomData, DeviceRef, Disk, Machine, Rom, Sample, Software};
use crate::helpers::ui_helper::init_progress_bar;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;

/**
 * The `mame.dat` file format represents data about arcade machines and their components.
 * The following outlines the structure used for parsing this file:
 *
 * - `Machine`: Represents a single arcade machine with various attributes:
 *   - `name`: The unique identifier for the machine (attribute).
 *   - `source_file`: Optional source file for the machine's data (attribute).
 *   - `rom_of`: Optional the ROM depends on files from another ROM to function correctly. (attribute).
 *   - `clone_of`: Optional the ROM is a modified version or variant of another ROM known as the parent ROM. (attribute).
 *   - `is_bios`: Optional flag indicating if the machine is a BIOS (attribute).
 *   - `is_device`: Optional flag indicating if the machine is a device (attribute).
 *   - `runnable`: Optional flag indicating if the machine is runnable (attribute).
 *   - `is_mechanical`: Optional flag indicating if the machine is mechanical (attribute).
 *   - `sample_of`: Optional the ROM uses specific sound samples from another ROM. (attribute).
 *   - `description`: Optional textual description of the machine (child node).
 *   - `year`: Optional year of release (child node).
 *   - `manufacturer`: Optional manufacturer name (child node).
 *   - `bios_sets`: Optional list of BIOS sets related to the machine (child nodes).
 *     - Each `<biosset>` element has:
 *       - `name`: Name of the BIOS set (attribute).
 *       - `description`: Description of the BIOS set (attribute).
 *   - `roms`: Optional list of ROMs associated with the machine (child nodes).
 *     - Each `<rom>` element has:
 *       - `name`: Name of the ROM (attribute).
 *       - `size`: Size of the ROM (attribute).
 *       - `merge`: Optional merge attribute (attribute).
 *       - `status`: Optional status attribute (attribute).
 *       - `crc`: Optional CRC value (attribute).
 *       - `sha1`: Optional SHA1 value (attribute).
 *   - `device_refs`: Optional list of device references related to the machine (child nodes).
 *     - Each `<device_ref>` element has:
 *       - `name`: Name of the device reference (attribute).
 *   - `software_list`: Optional list of software associated with the machine (child nodes).
 *     - Each `<softwarelist>` element has:
 *       - `name`: Name of the software (attribute).
 *   - `samples`: Optional list of samples associated with the machine (child nodes).
 *     - Each `<sample>` element has:
 *       - `name`: Name of the sample (attribute).
 *   - `driver_status`: Optional status of the machine's driver (child node).
 *   - `disks`: Optional list of disks related to the machine (child nodes).
 *     - Each `<disk>` element has:
 *       - `name`: Name of the disk (attribute).
 *       - `sha1`: Optional SHA1 value (attribute).
 *       - `merge`: Optional merge attribute (attribute).
 *       - `status`: Optional status attribute (attribute).
 *       - `region`: Optional region attribute (attribute).
 */

/**
 * Read the contents of the given MAME XML file and populate the given HashMap with the machines.
 */
pub fn read_mame_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut machines = MACHINES.lock().unwrap();

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the file content
    let file_content = fs::read_to_string(file_path)?;

    // Count the number of machines in the file
    let total_elements = count_total_elements(&file_content)?;
    let pb = init_progress_bar(total_elements as u64, "machines in mame.dat");

    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.trim_text(true);

    let mut buf = Vec::with_capacity(8 * 1024);

    let mut current_machine: Option<Machine> = None;

    let mut processed_count = 0;
    let batch = 5000;

    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_machine)?;
            }
            Ok(Event::Empty(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_machine)?;
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"machine" => {
                    if let Some(machine) = current_machine.take() {
                        machines.insert(machine.name.clone(), machine);
                    }

                    processed_count += 1;
                    if processed_count % batch == 0 {
                        pb.inc(batch);
                    }
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => (),
        }
        buf.clear();
    }

    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();

    Ok(())
}

/**
 * Process the given XML node and update the current machine with the node's data.
 */
fn process_node(
    e: &quick_xml::events::BytesStart,
    reader: &mut Reader<BufReader<File>>,
    current_machine: &mut Option<Machine>,
) -> Result<(), Box<dyn std::error::Error>> {
    match e.name() {
        b"machine" => {
            let mut machine = Machine {
                name: String::new(),
                source_file: None,
                rom_of: None,
                clone_of: None,
                is_bios: None,
                is_device: None,
                runnable: None,
                is_mechanical: None,
                sample_of: None,
                description: None,
                year: None,
                manufacturer: None,
                bios_sets: vec![],
                roms: vec![],
                device_refs: vec![],
                software_list: vec![],
                samples: vec![],
                driver_status: None,
                languages: vec![],
                players: None,
                series: None,
                genre: None,
                subgenre: None,
                is_mature: None,
                history_sections: vec![],
                disks: vec![],
                custom_data: None,
            };
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => machine.name = attr.unescape_and_decode_value(reader)?,
                    b"sourcefile" => {
                        machine.source_file = Some(attr.unescape_and_decode_value(reader)?)
                    }
                    b"romof" => machine.rom_of = Some(attr.unescape_and_decode_value(reader)?),
                    b"cloneof" => machine.clone_of = Some(attr.unescape_and_decode_value(reader)?),
                    b"isbios" => {
                        machine.is_bios = Some(attr.unescape_and_decode_value(reader)? == "yes")
                    }
                    b"isdevice" => {
                        machine.is_device = Some(attr.unescape_and_decode_value(reader)? == "yes")
                    }
                    b"runnable" => {
                        machine.runnable = Some(attr.unescape_and_decode_value(reader)? == "yes")
                    }
                    b"ismechanical" => {
                        machine.is_mechanical =
                            Some(attr.unescape_and_decode_value(reader)? == "yes")
                    }
                    b"sampleof" => {
                        machine.sample_of = Some(attr.unescape_and_decode_value(reader)?)
                    }
                    _ => {}
                }
            }
            if machine.custom_data.is_none() {
                machine.custom_data = Some(CustomData::default());
            }
            machine.custom_data.as_mut().unwrap().is_parent = Some(true);
            if machine.clone_of.is_some() || machine.sample_of.is_some() {
                machine.custom_data.as_mut().unwrap().is_parent = Some(false);
            }
            *current_machine = Some(machine);
        }
        b"description" => {
            if let Some(ref mut machine) = current_machine {
                machine.description = Some(reader.read_text(b"description", &mut Vec::new())?);
            }
        }
        b"year" => {
            if let Some(ref mut machine) = current_machine {
                machine.year = Some(reader.read_text(b"year", &mut Vec::new())?);
                // If year contains ? then set year in Custom Data as Unknown
                if machine.year.as_ref().unwrap().contains('?') {
                    machine.custom_data.as_mut().unwrap().year = Some("Unknown".to_string());
                } else {
                    machine.custom_data.as_mut().unwrap().year = machine.year.clone();
                }
            }
        }
        b"manufacturer" => {
            if let Some(ref mut machine) = current_machine {
                machine.manufacturer = Some(reader.read_text(b"manufacturer", &mut Vec::new())?);
            }
        }
        b"biosset" => {
            let mut bios_set = BiosSet {
                name: String::new(),
                description: String::new(),
            };

            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => bios_set.name = attr.unescape_and_decode_value(reader)?,
                    b"description" => {
                        bios_set.description = attr.unescape_and_decode_value(reader)?
                    }
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.bios_sets.push(bios_set);
            }
        }
        b"rom" => {
            let mut rom = Rom {
                name: String::new(),
                merge: None,
                size: 0,
                crc: None,
                sha1: None,
                status: None,
            };
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => rom.name = attr.unescape_and_decode_value(reader)?,
                    b"merge" => rom.merge = Some(attr.unescape_and_decode_value(reader)?),
                    b"size" => {
                        rom.size = attr.unescape_and_decode_value(reader)?.parse().unwrap_or(0)
                    }
                    b"crc" => rom.crc = Some(attr.unescape_and_decode_value(reader)?),
                    b"sha1" => rom.sha1 = Some(attr.unescape_and_decode_value(reader)?),
                    b"status" => rom.status = Some(attr.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.roms.push(rom);
            }
        }
        b"device_ref" => {
            let mut device_ref = DeviceRef {
                name: String::new(),
            };

            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => device_ref.name = attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.device_refs.push(device_ref);
            }
        }
        b"softwarelist" => {
            let mut software = Software {
                name: String::new(),
            };

            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => software.name = attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.software_list.push(software);
            }
        }
        b"sample" => {
            let mut sample = Sample {
                name: String::new(),
            };

            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => sample.name = attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.samples.push(sample);
            }
        }
        b"disk" => {
            let mut disk = Disk {
                name: String::new(),
                sha1: None,
                merge: None,
                status: None,
                region: None,
            };
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => disk.name = attr.unescape_and_decode_value(reader)?,
                    b"sha1" => disk.sha1 = Some(attr.unescape_and_decode_value(reader)?),
                    b"merge" => disk.merge = Some(attr.unescape_and_decode_value(reader)?),
                    b"status" => disk.status = Some(attr.unescape_and_decode_value(reader)?),
                    b"region" => disk.region = Some(attr.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.disks.push(disk);
            }
        }
        b"driver" => {
            let mut driver_status = String::new();
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"status" => driver_status = attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            if let Some(ref mut machine) = current_machine {
                machine.driver_status = Some(driver_status);
            }
        }
        _ => (),
    }

    Ok(())
}

/**
 * Count the number of machines in the given MAME XML file content.
 */
pub fn count_total_elements(file_content: &str) -> Result<usize, Box<dyn Error>> {
    let mut reader = Reader::from_str(file_content);
    reader.trim_text(true);
    let mut buf = Vec::with_capacity(8 * 1024);
    let mut count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"machine" => {
                count += 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                // Return the error instead of printing it
                return Err(Box::new(e));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(count)
}
