use crate::core::data::MACHINES;
use crate::core::models::Resource;
use crate::helpers::ui_helper::init_progress_bar;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;

/**
 * The `resources.dat` file format represents a structured dataset of various resources associated with arcade machines.
 * The structure is organized into `machine` elements, each representing a different resource grouping.
 * Below is the outline of the structure used for parsing this file:
 *
 * - `Machine`: Represents a resource group associated with a specific machine:
 *   - `name`: The unique identifier for the machine or resource group (attribute).
 *     - Possible values include: `artpreview`, `bosses`, `cabinets`, `covers`, `cpanel`, `devices`, 
 *       `ends`, `flyers`, `gameover`, `howto`, `icons`, `logo`, `manuals`, `marquees`, `pcb`, 
 *       `scores`, `select`, `snap`, `titles`, `versus`, `videosnaps`, `warning`.
 *
 *   - `description`: A textual description of the resource group (child node).
 *
 *   - `roms`: A collection of `rom` elements, each representing a specific resource file associated with the machine (child nodes).
 *     - Each `<rom>` element has the following attributes:
 *       - `name`: The name of the resource file including the file path (e.g., `artpreview\005.png`).
 *       - `size`: The size of the resource file in bytes.
 *       - `crc`: The CRC32 checksum of the resource file, used for integrity verification.
 *       - `sha1`: The SHA1 hash of the resource file, providing a more secure integrity check.
 *
 * - `machine`: Each machine element groups together a set of related resources, identified by the `name` attribute.
 * - `description`: Provides a brief textual description of the machine or resource group.
 * - `rom`: Represents individual resource files, associated with artwork, snapshots, or other media related to the arcade machine.
 *
 * This format is used to organize and reference additional content that can be associated with arcade machines in an emulation environment, making it easier to manage large collections of resources.
 */

/**
 * Read the resources file and add the information to the machines.
 */
pub fn read_resources_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the file content
    let file_content = fs::read_to_string(file_path)?;

    // Count the number of roms in each machine section in the file
    let total_elements = count_total_elements(&file_content)?;
    let pb = init_progress_bar(total_elements as u64, "roms in resources dat file");

    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.trim_text(true);

    let mut buf = Vec::with_capacity(8 * 1024);

    let mut current_section: Option<String> = None;

    let mut processed_count = 0;
    let batch = 5000;

    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_section)?;
            }
            Ok(Event::Empty(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_section)?;
                processed_count += 1;
                if processed_count % batch == 0 {
                    pb.inc(batch);
                }
            }
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
* Process the given XML node.
 */
fn process_node(
    e: &quick_xml::events::BytesStart,
    reader: &mut Reader<BufReader<File>>,
    current_section: &mut Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match e.name() {
        b"machine" => {
            let mut section_name: Option<String> = None;
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => section_name = Some(attr.unescape_and_decode_value(reader)?),
                    _ => {}
                }
            }
            
            *current_section = section_name;
        }
        b"rom" => {
            let mut resource = Resource {
                type_:String::new(),
                name: String::new(),
                size: 0,
                crc: String::new(),
                sha1: String::new(),
            };
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => resource.name = attr.unescape_and_decode_value(reader)?,
                    b"size" => {
                        resource.size = attr.unescape_and_decode_value(reader)?.parse().unwrap_or(0)
                    }
                    b"crc" => resource.crc = attr.unescape_and_decode_value(reader)?,
                    b"sha1" => resource.sha1 =attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            // Get the machine name based on the rom name
            let splitted = resource.name.split("\\").collect::<Vec<&str>>();

            if splitted.len() < 2 {
                return Ok(());
            }

            let resource_type = splitted[0].to_string();
            let machine_name = splitted[1].split(".").next().unwrap_or_default();

            // If exists section name then add the information to the machine if the machine exists
            if let Some(section_name) = current_section {
                // Check if the resource type is the same as the section name
                // Avoid adding non arcade resources to the machines
                if *section_name == resource_type {
                    if let Some(machine) = MACHINES.lock().unwrap().get_mut(machine_name) {
                        resource.type_ = section_name.clone();
                        machine.resources.push(resource);
                    }    
                }
            }
        }
        _ => (),
    }

    Ok(())
}

/**
 * Count the number of roms in the given XML file content.
 */
pub fn count_total_elements(file_content: &str) -> Result<usize, Box<dyn Error>> {
    let mut reader = Reader::from_str(file_content);
    reader.trim_text(true);
    let mut buf = Vec::with_capacity(8 * 1024);
    let mut count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) if e.name() == b"rom" => {
                count += 1;
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(Box::new(e));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(count)
}