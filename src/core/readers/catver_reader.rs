use crate::core::data::MACHINES;
use crate::helpers::ui_helper::init_progress_bar;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * The `catver.ini` file represents configurations and data related to game classification in the MAME system.
 * The file is organized into lines, where each line corresponds to a game entry with its category and subcategory.
 *
 * The file structure is as follows:
 *
 * - `[FOLDER_SETTINGS]`: An optional section for folder settings.
 *   - `RootFolderIcon`: Specifies the icon for the root folder.
 *   - `SubFolderIcon`: Specifies the icon for sub-folders.
 *
 * - `[ROOT_FOLDER]`: A placeholder section for root folder configurations (may be empty).
 *
 * - `<ROM Name>=<Category> / <Subcategory> * Mature *`
 *   - `<ROM Name>`: The name of the ROM being configured.
 *   - `<Category>`: The category of the game.
 *   - `<Subcategory>`: The subcategory of the game, which may be followed by `* Mature *` if the game is marked as mature.
 *
 * Note: The `category` and `subcategory` are separated by ` / `, and the subcategory may or may not end with the `* Mature *` marker.
 */

/**
 * Read the catver.ini file and update the machines with the category, subcategory, and is_mature values.
 */
pub fn read_catver_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();
    let total_elements = count_total_elements(file_path)?;
    let pb = init_progress_bar(total_elements as u64, "roms in catver.ini");

    let to_ignore = ["[", ";", "", " "];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut processed_count = 0;
    let batch = 1000;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        let first_char = trimmed.chars().next().unwrap_or(' ');

        if to_ignore.contains(&first_char.to_string().as_str()) {
            continue;
        }

        if let Some(equal_pos) = trimmed.find('=') {
            let (rom_name, value) = trimmed.split_at(equal_pos);
            let rom_name = rom_name.trim();
            let value = &value[1..].trim(); // Skip the '=' and trim the value

            let parts: Vec<&str> = value.split(" / ").collect();
            if parts.len() >= 2 {
                let category = parts[0].to_string();
                let mut subcategory = parts[1].to_string();
                let is_mature = subcategory.ends_with(" * Mature *");

                if is_mature {
                    subcategory = subcategory
                        .trim_end_matches(" * Mature *")
                        .trim()
                        .to_string();
                }

                if let Some(machine) = machines.get_mut(rom_name) {
                    machine.category = Some(category);
                    machine.subcategory = Some(subcategory);
                    machine.is_mature = Some(is_mature);
                }
            }
            processed_count += 1;
            if processed_count % batch == 0 {
                pb.inc(batch);
            }
        }
    }

    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();
    Ok(())
}

/**
 * Count the total elements in the file.
 */
fn count_total_elements(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.contains('=') {
            count += 1;
        }
    }

    Ok(count)
}
