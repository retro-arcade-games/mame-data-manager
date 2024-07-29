use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use indicatif::{ProgressBar, ProgressStyle};
use crate::models::Machine;

/**
 * The `catver.ini` file represents configurations and data related to game classification in the MAME system.
 * The file is organized into lines, where each line corresponds to a game entry with its genre and subgenre.
 *
 * The file structure is as follows:
 *
 * - `[FOLDER_SETTINGS]`: An optional section for folder settings.
 *   - `RootFolderIcon`: Specifies the icon for the root folder.
 *   - `SubFolderIcon`: Specifies the icon for sub-folders.
 * 
 * - `[ROOT_FOLDER]`: A placeholder section for root folder configurations (may be empty).
 *
 * - `<ROM Name>=<Genre> / <Subgenre> * Mature *`
 *   - `<ROM Name>`: The name of the ROM being configured.
 *   - `<Genre>`: The genre of the game.
 *   - `<Subgenre>`: The subgenre of the game, which may be followed by `* Mature *` if the game is marked as mature.
 *
 * Note: The `genre` and `subgenre` are separated by ` / `, and the subgenre may or may not end with the `* Mature *` marker.
 */


/**
 * Read the catver.ini file and update the machines with the genre, subgenre, and is_mature values.
 */
pub fn read_catver_file(file_path: &str, machines: &mut HashMap<String, Machine>) -> Result<(), Box<dyn Error>>{
    let total_elements = count_total_elements(file_path)?;
    let pb = ProgressBar::new(total_elements as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} roms in catver.ini ({eta})")
            .progress_chars("#>-"),
    );

    let to_ignore = ["[", ";", "", " "];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

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
                let genre = parts[0].to_string();
                let mut subgenre = parts[1].to_string();
                let is_mature = subgenre.ends_with(" * Mature *");

                if is_mature {
                    subgenre = subgenre.trim_end_matches(" * Mature *").trim().to_string();
                }

                if let Some(machine) = machines.get_mut(rom_name) {
                    machine.genre = Some(genre);
                    machine.subgenre = Some(subgenre);
                    machine.is_mature = Some(is_mature);
                    pb.inc(1);
                }
            }
        }
    }

    pb.finish_with_message("Processing complete");
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