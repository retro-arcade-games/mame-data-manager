use crate::models::Machine;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * The `languages.ini` file format represents configurations and data related to different languages in the system.
 * The file is organized into sections, where each section corresponds to a specific language.
 * Within each language section, entries represent names of ROMs associated with that language.
 *
 * - `[FOLDER_SETTINGS]`: A section for folder settings.
 *   - `RootFolderIcon`: Specifies the icon for the root folder.
 *   - `SubFolderIcon`: Specifies the icon for sub-folders.
 * 
 * - `[ROOT_FOLDER]`: A placeholder section for root folder configurations (may be empty).
 *
 * - `[<Language>]`: Sections where each section header is a language identifier.
 *   - Entries: Each entry is a ROM name associated with the specific language.
 *
 * Note: Sections are labeled by language names, and the entries under each section are ROM names associated with that language.
 */


/**
 * Read the contents of the given languages file and populate the given HashMap with the languages.
 */
pub fn read_languages_file(file_path: &str, machines: &mut HashMap<String, Machine>) {
    let total_elements = count_total_elements(file_path).expect("Failed to count total elements");
    let pb = ProgressBar::new(total_elements as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} lines ({eta})")
            .progress_chars("#>-"),
    );

    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut current_language: Option<String> = None;

    let to_ignore = vec![";", "", " ", "", "[FOLDER_SETTINGS]", "[ROOT_FOLDER]"];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let first_char = line.chars().next().unwrap_or(' ');

        if !to_ignore.contains(&first_char.to_string().as_str()) && !to_ignore.contains(&line.as_str()) {
            if first_char == '[' {
                current_language = Some(line.replace("[", "").replace("]", ""));
            } else if let Some(language) = &current_language {
                if let Some(machine) = machines.get_mut(&line) {
                    machine.languages.push(language.clone());
                }
            }
        }

        pb.inc(1);
    }

    pb.finish_with_message("Processing complete");
}

/**
 * Count the total number of elements in the given languages file.
 */
fn count_total_elements(file_path: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let to_ignore = vec![
        ";",
		"",
		" ",
		"",
		"[FOLDER_SETTINGS]",
		"[ROOT_FOLDER]",
		"[",
		"RootFolderIcon mame",
		"SubFolderIcon folder",
    ];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let count = reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            let first_char = line.chars().next().unwrap_or(' ');
            !to_ignore.contains(&line.as_str()) && !to_ignore.contains(&first_char.to_string().as_str())
        })
        .count();

    Ok(count)
}
