use crate::models::Machine;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
 * The `nplayers.ini` file format represents configurations related to the number of players and game types for various ROMs.
 * The file is organized into a single section `[NPlayers]`, where each entry corresponds to a specific ROM and its associated player count or game type.
 * Each line follows the format:
 *
 * - `ROM_Name=Player_Count_Or_Game_Type`
 *
 * Where:
 * - `ROM_Name`: The name of the ROM file.
 * - `Player_Count_Or_Game_Type`: Describes the number of players or the type of game associated with the ROM.
 *
 * Possible values for `Player_Count_Or_Game_Type` include:
 *
 * - `1P`: Single-player game.
 * - `2P alt`: Alternate two-player mode.
 * - `2P sim`: Simultaneous two-player mode.
 * - `3P sim`: Simultaneous three-player mode.
 * - `3P alt`: Alternate three-player mode.
 * - `4P alt`: Alternate four-player mode.
 * - `4P sim`: Simultaneous four-player mode.
 * - `4P alt / 2P sim`: Alternate four-player mode or simultaneous two-player mode.
 * - `5P alt`: Alternate five-player mode.
 * - `6P alt`: Alternate six-player mode.
 * - `6P sim`: Simultaneous six-player mode.
 * - `6P alt / 2P sim`: Alternate six-player mode or simultaneous two-player mode.
 * - `8P alt`: Alternate eight-player mode.
 * - `8P alt / 2P sim`: Alternate eight-player mode or simultaneous two-player mode.
 * - `9P alt`: Alternate nine-player mode.
 * - `Pinball`: Pinball game.
 * - `BIOS`: BIOS or system ROM.
 * - `Device`: Non-playable device.
 * - `Non-arcade`: Non-arcade game.
 * - `???`: Unknown or unspecified number of players.
 *
 * Lines that start with `[` or `;`, or are empty, are considered comments or section headers and are ignored.
 */

/**
 * Read the nplayers.ini file and update the machines with the number of players
 */
pub fn read_nplayers_file(
    file_path: &str,
    machines: &mut HashMap<String, Machine>,
) -> Result<(), Box<dyn Error>> {
    let total_elements = count_total_elements(file_path)?;
    let pb = ProgressBar::new(total_elements as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} roms in nplayers.ini ({eta})")
            .progress_chars("#>-"),
    );

    let to_ignore = ["[", ";", "", " "];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        let first_char = trimmed.chars().next().unwrap_or(' ');

        // Skip lines that start with any of the ignore characters or patterns
        if to_ignore.contains(&first_char.to_string().as_str()) {
            continue;
        }

        // Process lines with '=' sign
        if let Some(equal_pos) = trimmed.find('=') {
            let (rom_name, value) = trimmed.split_at(equal_pos);
            let rom_name = rom_name.trim();
            let value = &value[1..].trim(); // Skip the '=' and trim the value

            if let Some(machine) = machines.get_mut(rom_name) {
                // Update machine.players with the value from the file
                machine.players = Some(value.to_string());
                pb.inc(1);
            }
        }
    }

    pb.finish_and_clear();
    Ok(())
}

/**
 * Count the total number of elements in the file
 */
fn count_total_elements(file_path: &str) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        if line.contains('=') {
            count += 1;
        }
    }

    Ok(count)
}
