use crate::models::{Machine, BiosSet, Rom, DeviceRef, Software, Sample};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::str::from_utf8;

pub fn read_mame_file(file_path: &str, machines: &mut HashMap<String, Machine>) {
    let file_content = fs::read_to_string(file_path).expect("Unable to read file");

    // Contar el número total de máquinas
    let total_machines = count_machines(&file_content);
    let pb = ProgressBar::new(total_machines as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} machines ({eta})",
            )
            .progress_chars("#>-"),
    );

    let mut reader = Reader::from_str(&file_content);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut current_machine: Option<Machine> = None;
    let mut current_element: Option<String> = None;
    let mut current_text = String::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let name = from_utf8(e.name()).unwrap();
                if name == "machine" {
                    current_machine = Some(parse_machine_element(e.attributes().map(|a| a.unwrap()).collect(), &reader));
                } else if let Some(ref mut machine) = current_machine {
                    current_element = Some(name.to_string());
                    current_text.clear();  // Clear text buffer for new element
                    parse_child_element(name, e.attributes().map(|a| a.unwrap()).collect(), machine, None);
                }
            }
            Ok(Event::End(ref e)) => {
                let name = from_utf8(e.name()).unwrap();
                if name == "machine" {
                    if let Some(machine) = current_machine.take() {
                        machines.insert(machine.name.clone(), machine);
                        pb.inc(1);
                    }
                } else if let Some(ref mut machine) = current_machine {
                    // Handle text content of the element
                    if let Some(ref element) = current_element {
                        parse_child_element(element, vec![], machine, Some(&current_text));
                    }
                }
                current_element = None;
            }
            Ok(Event::Text(e)) => {
                current_text.push_str(&e.unescape_and_decode(&reader).unwrap()); // Collect characters for the current element
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    pb.finish_with_message("Processing complete");
}

fn count_machines(file_content: &str) -> usize {
    let mut reader = Reader::from_str(file_content);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name() == b"machine" {
                    count += 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    count
}

fn parse_machine_element(attributes: Vec<quick_xml::events::attributes::Attribute>, reader: &Reader<&[u8]>) -> Machine {
    let mut machine = Machine {
        name: String::new(),
        source_file: None,
        rom_of: None,
        clone_of: None,
        is_bios: None,
        is_device: None,
        runnable: None,
        is_mechanical: false,
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
    };

    for attr in attributes {
        match from_utf8(attr.key).unwrap() {
            "name" => machine.name = attr.unescape_and_decode_value(reader).unwrap(),
            "sourcefile" => machine.source_file = Some(attr.unescape_and_decode_value(reader).unwrap()),
            "romof" => machine.rom_of = Some(attr.unescape_and_decode_value(reader).unwrap()),
            "cloneof" => machine.clone_of = Some(attr.unescape_and_decode_value(reader).unwrap()),
            "isbios" => machine.is_bios = Some(attr.unescape_and_decode_value(reader).unwrap() == "yes"),
            "isdevice" => machine.is_device = Some(attr.unescape_and_decode_value(reader).unwrap() == "yes"),
            "runnable" => machine.runnable = Some(attr.unescape_and_decode_value(reader).unwrap() == "yes"),
            "ismechanical" => machine.is_mechanical = attr.unescape_and_decode_value(reader).unwrap() == "yes",
            "sampleof" => machine.sample_of = Some(attr.unescape_and_decode_value(reader).unwrap()),
            _ => {}
        }
    }

    machine
}

fn parse_child_element(name: &str, attributes: Vec<quick_xml::events::attributes::Attribute>, machine: &mut Machine, text: Option<&str>) {
    match name {
        "description" => {
            if let Some(text) = text {
                machine.description = Some(text.to_string());
            }
        }
        "year" => {
            if let Some(text) = text {
                machine.year = Some(text.to_string());
            }
        }
        "manufacturer" => {
            if let Some(text) = text {
                machine.manufacturer = Some(text.to_string());
            }
        }
        "driver" => {
            if let Some(text) = text {
                machine.driver_status = Some(text.to_string());
            }
        }
        "biosset" => {
            if attributes.len() >= 2 {
                let bios_set = BiosSet {
                    name: from_utf8(&attributes[0].value).unwrap().to_string(),
                    description: from_utf8(&attributes[1].value).unwrap().to_string(),
                };
                machine.bios_sets.push(bios_set);
            }
        }
        "rom" => {
            if attributes.len() >= 5 {
                let rom = Rom {
                    name: from_utf8(&attributes[0].value).unwrap().to_string(),
                    merge: attributes.get(1).map(|a| from_utf8(&a.value).unwrap().to_string()),
                    size: from_utf8(&attributes[2].value).unwrap().parse().unwrap_or(0),
                    crc: Some(from_utf8(&attributes[3].value).unwrap().to_string()),
                    sha1: Some(from_utf8(&attributes[4].value).unwrap().to_string()),
                    status: attributes.get(5).map(|a| from_utf8(&a.value).unwrap().to_string()),
                };
                machine.roms.push(rom);
            }
        }
        "device_ref" => {
            if let Some(attr) = attributes.get(0) {
                let device_ref = DeviceRef {
                    name: from_utf8(&attr.value).unwrap().to_string(),
                };
                machine.device_refs.push(device_ref);
            }
        }
        "softwarelist" => {
            if let Some(attr) = attributes.get(0) {
                let software = Software {
                    name: from_utf8(&attr.value).unwrap().to_string(),
                };
                machine.software_list.push(software);
            }
        }
        "sample" => {
            if let Some(attr) = attributes.get(0) {
                let sample = Sample {
                    name: from_utf8(&attr.value).unwrap().to_string(),
                };
                machine.samples.push(sample);
            }
        }
        _ => {}
    }
}
