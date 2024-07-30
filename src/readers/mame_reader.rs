use crate::models::{Machine, BiosSet, Rom, DeviceRef, Software, Sample, Disk};
use indicatif::{ProgressBar, ProgressStyle};
use roxmltree::Document;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use quick_xml::events::Event;
use quick_xml::Reader;

/**
 * The `mame.dat` file format represents data about arcade machines and their components. 
 * The following outlines the structure used for parsing this file:
 *
 * - `Machine`: Represents a single arcade machine with various attributes:
 *   - `name`: The unique identifier for the machine (attribute).
 *   - `source_file`: Optional source file for the machine's data (attribute).
 *   - `rom_of`: Optional name of the ROM set that this machine is based on (attribute).
 *   - `clone_of`: Optional name of the ROM set this machine is a clone of (attribute).
 *   - `is_bios`: Optional flag indicating if the machine is a BIOS (attribute).
 *   - `is_device`: Optional flag indicating if the machine is a device (attribute).
 *   - `runnable`: Optional flag indicating if the machine is runnable (attribute).
 *   - `is_mechanical`: Optional flag indicating if the machine is mechanical (attribute).
 *   - `sample_of`: Optional name of the sample set associated with this machine (attribute).
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
pub fn read_mame_file(file_path: &str, machines: &mut HashMap<String, Machine>) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file content
    let file_content = fs::read_to_string(file_path)?;

    // Count the number of machines in the file
    let total_machines = count_total_elements(&file_content)?;
    let pb = ProgressBar::new(total_machines as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} machines in mame.dat ({eta})",
            )
            .progress_chars("#>-"),
    );

    // Parse XML document
    let doc = Document::parse(&file_content).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // Process each node in the XML document
    for node in doc.descendants() {
        if node.tag_name().name() == "machine" {
            let machine = parse_machine_element(&node);
            machines.insert(machine.name.clone(), machine);
            pb.inc(1);
        }
    }

    pb.finish_with_message("Processing complete");

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

/**
 * Parse the given machine element and return a Machine struct.
 */
fn parse_machine_element(node: &roxmltree::Node) -> Machine {
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
    };

    for attr in node.attributes() {
        match attr.name() {
            "name" => machine.name = attr.value().to_string(),
            "sourcefile" => machine.source_file = Some(attr.value().to_string()),
            "romof" => machine.rom_of = Some(attr.value().to_string()),
            "cloneof" => machine.clone_of = Some(attr.value().to_string()),
            "isbios" => machine.is_bios = Some(attr.value() == "yes"),
            "isdevice" => machine.is_device = Some(attr.value() == "yes"),
            "runnable" => machine.runnable = Some(attr.value() == "yes"),
            "ismechanical" => machine.is_mechanical = Some(attr.value() == "yes"),
            "sampleof" => machine.sample_of = Some(attr.value().to_string()),
            _ => {}
        }
    }

    for child in node.children() {
        if child.is_element() {
            parse_child_element(child.tag_name().name(), &child, &mut machine);
        }
    }

    machine
}

/**
 * Parse the given child element and update the given Machine struct.
 */
fn parse_child_element(name: &str, node: &roxmltree::Node, machine: &mut Machine) {
    match name {
        "description" => {
            if let Some(text) = node.text() {
                machine.description = Some(text.to_string());
            }
        }
        "year" => {
            if let Some(text) = node.text() {
                machine.year = Some(text.to_string());
            }
        }
        "manufacturer" => {
            if let Some(text) = node.text() {
                machine.manufacturer = Some(text.to_string());
            }
        }
        "driver" => {
            if let Some(status) = node.attribute("status") {
                machine.driver_status = Some(status.to_string());
            }
        }
        "biosset" => {
            let bios_set = BiosSet {
                name: node.attribute("name").unwrap_or("").to_string(),
                description: node.attribute("description").unwrap_or("").to_string(),
            };
            machine.bios_sets.push(bios_set);
        }
        "rom" => {
            let rom = Rom {
                name: node.attribute("name").unwrap_or("").to_string(),
                merge: node.attribute("merge").map(|s| s.to_string()),
                size: node.attribute("size").unwrap_or("0").parse().unwrap_or(0),
                crc: node.attribute("crc").map(|s| s.to_string()),
                sha1: node.attribute("sha1").map(|s| s.to_string()),
                status: node.attribute("status").map(|s| s.to_string()),
            };
            machine.roms.push(rom);
        }
        "device_ref" => {
            let device_ref = DeviceRef {
                name: node.attribute("name").unwrap_or("").to_string(),
            };
            machine.device_refs.push(device_ref);
        }
        "softwarelist" => {
            let software = Software {
                name: node.attribute("name").unwrap_or("").to_string(),
            };
            machine.software_list.push(software);
        }
        "sample" => {
            let sample = Sample {
                name: node.attribute("name").unwrap_or("").to_string(),
            };
            machine.samples.push(sample);
        }
        "disk" => {
            let disk = Disk {
                name: node.attribute("name").unwrap_or("").to_string(),
                sha1:node.attribute("sha1").map(|s| s.to_string()),
                merge: node.attribute("merge").map(|s| s.to_string()),
                region: node.attribute("region").map(|s| s.to_string()),
                status: node.attribute("status").map(|s| s.to_string()),
            };
            machine.disks.push(disk);
        }
        _ => {
            println!("Unknown element: {}", name); // Debugging line to catch any unknown elements
        }
    }
}
