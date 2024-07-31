use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use indicatif::{ProgressBar, ProgressStyle};
use roxmltree::Document;
use quick_xml::Reader;
use quick_xml::events::Event;
use crate::models::{HistorySection, Machine};

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
pub fn read_history_file(file_path: &str, machines: &mut HashMap<String, Machine>) -> Result<(), Box<dyn Error>> {
    // Read the entire file content into a string
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    // Get the total number of entries from the content
    let total_entries = count_total_elements(&content)?;
    let pb = ProgressBar::new(total_entries as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} entries in history.xml ({eta})")
            .progress_chars("#>-"),
    );

    // Parse the XML content
    let doc = Document::parse(&content)?;

    let mut current_entry = None;

    for node in doc.descendants() {
        match node.tag_name().name() {
            "entry" => {
                current_entry = Some(parse_entry(node));
            }
            _ => {}
        }

        if let Some(entry) = current_entry.take() {
            for name in entry.names {
                if let Some(machine) = machines.get_mut(&name) {
                    machine.history_sections = entry.sections.clone();
                }
            }
            pb.inc(1);
        }
    }

    pb.finish_and_clear();
    Ok(())
}

/**
 * Parse an <entry> node from the history XML file.
 */
fn parse_entry(node: roxmltree::Node) -> ParsedEntry {
    let mut names = Vec::new();
    let mut sections = Vec::new();
    let mut current_section_name = "description".to_string();

    for child in node.children() {
        match child.tag_name().name() {
            "systems" => {
                for system in child.children() {
                    if system.tag_name().name() == "system" {
                        if let Some(name) = system.attribute("name") {
                            names.push(name.to_string());
                        }
                    }
                }
            }
            "text" => {
                let text_content = child.text().unwrap_or("").trim();
                sections.extend(parse_text(text_content, &mut current_section_name));
            }
            _ => {}
        }
    }

    ParsedEntry { names, sections }
}

/**
 * Parse the text content of an <entry> node and return a list of HistorySection objects.
 */
fn parse_text(text: &str, current_section_name: &mut String) -> Vec<HistorySection> {
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
        "- CONTRIBUTE -"
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

            *current_section_name = line.to_string().replace('-', "").trim().to_lowercase();
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
struct ParsedEntry {
    names: Vec<String>,
    sections: Vec<HistorySection>,
}
