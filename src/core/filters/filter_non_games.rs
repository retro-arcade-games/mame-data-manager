use crate::{
    core::{data::MACHINES, models::Machine},
    helpers::ui_helper::init_progress_bar,
};
use std::error::Error;

/**
 * Filter out non-game machines from the collection
 */
pub fn filter_non_games() -> Result<u64, Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();
    let mut removed_machine_count: u64 = 0;
    let mut machines_to_remove: Vec<String> = Vec::new();

    let pb = init_progress_bar((machines.len() * 2) as u64, "finding machines to remove");

    let mut processed_count = 0;
    let batch = 5000;

    // Iterate the machines hashmap
    for (_, machine) in machines.iter_mut() {
        if is_non_game_machine(machine) {
            machines_to_remove.push(machine.name.clone());
            removed_machine_count += 1;
        }

        processed_count += 1;
        if processed_count % batch == 0 {
            pb.inc(batch);
        }
    }
    pb.set_message("removing machines");
    // Remove machines from collection
    for machine_name in machines_to_remove {
        machines.remove(&machine_name);
        processed_count += 1;
        if processed_count % batch == 0 {
            pb.inc(batch);
        }
    }

    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();

    Ok(removed_machine_count)
}

fn is_non_game_machine(machine: &Machine) -> bool {
    machine.is_device.unwrap_or(false)
        || machine.is_bios.unwrap_or(false)
        || machine.is_mechanical.unwrap_or(false)
        || is_modified_machine(&machine.description.as_ref().unwrap_or(&"".to_string()))
        || has_invalid_manufacturer(machine)
        || has_invalid_players(&machine)
}

/**
 * Check if the machine is a modified version by its description
 */
fn is_modified_machine(description: &str) -> bool {
    let modified_keywords = vec![
        "bootleg",
        "PlayChoice-10",
        "Nintendo Super System",
        "prototype",
    ];
    for keyword in modified_keywords {
        if description.to_lowercase().contains(&keyword.to_lowercase()) {
            return true;
        }
    }
    false
}

/**
 * Check if extended_data.manufacturer is invalid
 */
fn has_invalid_manufacturer(machine: &Machine) -> bool {
    let invalid_manufacturers = vec!["unknown", "bootleg"];
    // Check if machine has extended data
    if let Some(extended_data) = &machine.extended_data {
        // Check if manufacturer is invalid
        if let Some(manufacturer) = &extended_data.manufacturer {
            for invalid_manufacturer in invalid_manufacturers {
                if manufacturer
                    .to_lowercase()
                    .contains(&invalid_manufacturer.to_lowercase())
                {
                    return true;
                }
            }
        }
    }
    false
}

/**
 * Check if players is invalid
 */
fn has_invalid_players(machine: &Machine) -> bool {
    let invalid_players = vec!["BIOS", "Device", "Non-arcade"];
    if let Some(players) = &machine.players {
        for invalid_player in invalid_players {
            if players
                .to_lowercase()
                .contains(&invalid_player.to_lowercase())
            {
                return true;
            }
        }
    }
    false
}
