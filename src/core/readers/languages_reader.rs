use crate::core::models::Machine;
use crate::helpers::ui_helper::init_progress_bar;
use std::collections::HashMap;
use std::error::Error;
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
pub fn read_languages_file(
    file_path: &str,
    machines: &mut HashMap<String, Machine>,
) -> Result<(), Box<dyn Error>> {
    // Count the total number of elements for the progress bar
    let total_elements = count_total_elements(file_path)?;
    let pb = init_progress_bar(total_elements as u64, "roms in languages.ini");

    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut current_language: Option<String> = None;

    // Define lines to ignore
    let to_ignore = vec![";", "", " ", "", "[FOLDER_SETTINGS]", "[ROOT_FOLDER]"];

    // Process each line of the file
    for line in reader.lines() {
        let line = line?;
        let first_char = line.chars().next().unwrap_or(' ');

        if !to_ignore.contains(&first_char.to_string().as_str())
            && !to_ignore.contains(&line.as_str())
        {
            if first_char == '[' {
                // Set the current language when a new language section starts
                current_language = Some(line.replace("[", "").replace("]", ""));
            } else if let Some(language) = &current_language {
                // Update the machine's languages if the line matches a machine name
                if let Some(machine) = machines.get_mut(&line) {
                    machine.languages.push(language.clone());
                    pb.inc(1);
                }
            }
        }
    }

    pb.finish_and_clear();
    Ok(())
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

    let count = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            let first_char = line.chars().next().unwrap_or(' ');
            !to_ignore.contains(&line.as_str())
                && !to_ignore.contains(&first_char.to_string().as_str())
        })
        .count();

    Ok(count)
}
