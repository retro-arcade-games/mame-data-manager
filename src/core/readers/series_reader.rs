use crate::helpers::ui_helper::init_progress_bar;
use crate::core::models::Machine;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * The `series.ini` file format represents configurations and data related to different game series in the system.
 * The file is organized into sections, where each section corresponds to a specific game series.
 * Within each series section, entries represent names of ROMs associated with that series.
 *
 * - `[FOLDER_SETTINGS]`: A section for folder settings.
 *   - `RootFolderIcon`: Specifies the icon for the root folder.
 *   - `SubFolderIcon`: Specifies the icon for sub-folders.
 *
 * - `[ROOT_FOLDER]`: A placeholder section for root folder configurations (may be empty).
 *
 * - `[<Series>]`: Sections where each section header is a game series identifier.
 *   - Entries: Each entry is a ROM name associated with the specific game series.
 *
 * Note: Sections are labeled by series names, and the entries under each section are ROM names associated with that series.
 */

/**
 * Read the contents of the given series file and populate the given HashMap with the series.
 */
pub fn read_series_file(
    file_path: &str,
    machines: &mut HashMap<String, Machine>,
) -> Result<(), Box<dyn Error>> {
    let total_elements = count_total_elements(file_path)?;
    let pb = init_progress_bar(total_elements as u64, "roms in series.ini");

    let to_ignore = [";", "", " ", "", "[FOLDER_SETTINGS]", "[ROOT_FOLDER]"];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut current_series: Option<String> = None;

    for line in reader.lines() {
        let line = line?;

        let first_char = line.chars().next().unwrap_or(' ');

        if !to_ignore.contains(&line.as_str())
            && !to_ignore.contains(&first_char.to_string().as_str())
        {
            if first_char == '[' {
                current_series = Some(line.trim_matches(|c| c == '[' || c == ']').to_string());
            } else if let Some(series) = &current_series {
                if let Some(machine) = machines.get_mut(&line) {
                    machine.series = Some(series.clone());
                    pb.inc(1);
                }
            }
        }
    }

    pb.finish_and_clear();
    Ok(())
}

/**
 * Count the total number of elements in the given series file.
 */
fn count_total_elements(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let to_ignore = [
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
        .filter_map(Result::ok)
        .filter(|line| {
            !to_ignore.contains(&line.as_str())
                && !to_ignore.contains(&line.get(0..1).unwrap_or(""))
        })
        .count();

    Ok(count)
}
