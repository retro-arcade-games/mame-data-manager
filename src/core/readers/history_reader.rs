use crate::core::models::{HistorySection, Machine};
use crate::helpers::ui_helper::init_progress_bar;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;

/**
 * The XML file follows this general structure:
 *
 * <entry>
 *     <systems>
 *         <!-- Contains a list of systems that run the game. Each system is represented by a <system> element with the 'name' attribute. -->
 *         <system name="system_name" />
 *         <!-- ... other systems ... -->
 *     </systems>
 *
 *     <software>
 *         <!-- Contains information about software related to the game. Each software item is represented by an <item> element with 'list' and 'name' attributes. -->
 *         <item list="list_name" name="software_name" />
 *     </software>
 *
 *     <text>
 *         <!-- Contains various sections of text about the game. The possible sections are: -->
 *         <!-- - DESCRIPTION: Provides a general description of the game. -->
 *         <!-- - TECHNICAL: Details technical aspects or specifications of the game. -->
 *         <!-- - TRIVIA: Contains trivia or interesting facts about the game. -->
 *         <!-- - UPDATES: Lists updates or changes made to the game. -->
 *         <!-- - SCORING: Details on scoring or how the game is scored. -->
 *         <!-- - TIPS AND TRICKS: Offers tips and tricks for playing the game. -->
 *         <!-- - SERIES: Information about the game series or franchise. -->
 *         <!-- - STAFF: Lists the staff or developers involved with the game. -->
 *         <!-- - PORTS: Details on different ports or versions of the game. -->
 *         <!-- - CONTRIBUTE: Information on how to contribute or support the game. -->
 *     </text>
 * </entry>
 */

/**
 * Read the contents of the given history XML file and populate the given HashMap with the machines.
 */
pub fn read_history_file(
    file_path: &str,
    machines: &mut HashMap<String, Machine>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the file content
    let file_content = fs::read_to_string(file_path)?;

    // Count the number of machines in the file
    let total_elements = count_total_elements(&file_content)?;
    let pb = init_progress_bar(total_elements as u64, "entries in history.xml");

    let mut xml_reader = Reader::from_reader(reader);
    xml_reader.trim_text(true);

    let mut buf = Vec::with_capacity(8 * 1024);

    let mut current_entry: Option<HistoryEntry> = None;

    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_entry)?;
            }
            Ok(Event::Empty(ref e)) => {
                process_node(e, &mut xml_reader, &mut current_entry)?;
            }
            Ok(Event::End(ref e)) => match e.name() {
                b"entry" => {
                    if let Some(entry) = current_entry.take() {
                        for name in entry.names {
                            if let Some(machine) = machines.get_mut(&name) {
                                machine.history_sections = entry.sections.clone();
                            }
                        }
                        pb.inc(1);
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

    pb.finish_and_clear();

    Ok(())
}

/**
 * Process a node in the history XML file.
 */
fn process_node(
    e: &quick_xml::events::BytesStart,
    reader: &mut Reader<BufReader<File>>,
    current_entry: &mut Option<HistoryEntry>,
) -> Result<(), Box<dyn std::error::Error>> {
    match e.name() {
        b"entry" => {
            let entry = HistoryEntry {
                names: Vec::new(),
                sections: Vec::new(),
            };
            *current_entry = Some(entry);
        }
        b"system" => {
            let mut system_name = String::new();
            let attrs = e.attributes().map(|a| a.unwrap());
            for attr in attrs {
                match attr.key {
                    b"name" => system_name = attr.unescape_and_decode_value(reader)?,
                    _ => {}
                }
            }
            if let Some(ref mut entry) = current_entry {
                entry.names.push(system_name.clone());
            }
        }
        b"text" => {
            let text = reader.read_text(b"text", &mut Vec::new())?;
            let sections = parse_text(&text);
            if let Some(ref mut entry) = current_entry {
                entry.sections = sections;
            }
        }
        _ => (),
    }

    Ok(())
}

/**
 * Parse the text content of an <entry> node and return a list of HistorySection objects.
 */
fn parse_text(text: &str) -> Vec<HistorySection> {
    let mut current_section_name = String::new();
    let mut sections = Vec::new();
    let document_sections = [
        "- DESCRIPTION -",
        "- TECHNICAL -",
        "- TRIVIA -",
        "- UPDATES -",
        "- SCORING -",
        "- TIPS AND TRICKS -",
        "- SERIES -",
        "- STAFF -",
        "- PORTS -",
        "- CONTRIBUTE -",
    ];

    let mut current_section_text = String::new();
    let mut order = 1;

    for line in text.lines() {
        if document_sections.contains(&line) {
            if !current_section_text.is_empty() {
                sections.push(HistorySection {
                    name: current_section_name.clone(),
                    text: current_section_text.trim().to_string(),
                    order,
                });
                current_section_text.clear();
            }

            current_section_name = line.to_string().replace('-', "").trim().to_lowercase();
            order = get_section_order(line);
        } else {
            current_section_text.push_str(&(line.to_string() + "\n"));
        }
    }

    if !current_section_text.is_empty() {
        sections.push(HistorySection {
            name: current_section_name.clone(),
            text: current_section_text.trim().to_string(),
            order,
        });
    }

    sections
}

/**
 * Get the order of a section based on its name.
 */
fn get_section_order(section: &str) -> usize {
    match section {
        "- DESCRIPTION -" => 1,
        "- TECHNICAL -" => 2,
        "- TRIVIA -" => 3,
        "- UPDATES -" => 4,
        "- SCORING -" => 5,
        "- TIPS AND TRICKS -" => 6,
        "- SERIES -" => 7,
        "- STAFF -" => 8,
        "- PORTS -" => 9,
        "- CONTRIBUTE -" => 10,
        _ => 1,
    }
}

/**
 * Count the total number of <entry> elements in the history XML file.
 */
pub fn count_total_elements(file_content: &str) -> Result<usize, Box<dyn Error>> {
    let mut reader = Reader::from_str(file_content);
    reader.trim_text(true);
    let mut buf = Vec::with_capacity(8 * 1024);
    let mut count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) if e.name() == b"entry" => {
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

#[derive(Debug)]
struct HistoryEntry {
    names: Vec<String>,
    sections: Vec<HistorySection>,
}
