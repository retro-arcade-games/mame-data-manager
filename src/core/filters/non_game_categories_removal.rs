use crate::{core::data::MACHINES, helpers::ui_helper::init_progress_bar};
use std::error::Error;

/**
 * Filter out machines with categories that are not relevant to arcade machines
 */
pub fn remove_non_game_categories() -> Result<u64, Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();

    // If the machines were not loaded, return an error
    if machines.is_empty() {
        return Err("No machines data loaded, please read the data first.".into());
    }

    // List of categories to ignore
    let ignored_categories = vec![
        "Board Game",
        "Calculator",
        "Card Games",
        "Casino",
        "Computer",
        "Computer Graphic Workstation",
        "Digital Camera",
        "Digital Simulator",
        "Electromechanical",
        "Game",
        "Game Console",
        "Game Console/Computer",
        "Handheld",
        "Medical Equipment",
        "Misc.",
        "MultiGame",
        "Multiplay",
        "Music",
        "Player",
        "Printer",
        "Radio",
        "Rhythm",
        "Simulation",
        "Slot Machine",
        "System",
        "Tablet",
        "Tabletop",
        "Telephone",
        "Touchscreen",
        "TTL * Ball & Paddle",
        "TTL * Driving",
        "TTL * Maze",
        "TTL * Quiz",
        "TTL * Shooter",
        "TTL * Sports",
        "TV Bundle",
        "Utilities",
        "Watch",
    ];

    let mut removed_machine_count: u64 = 0;
    let mut machines_to_remove: Vec<String> = Vec::new();

    let pb = init_progress_bar((machines.len() * 2) as u64, "finding machines to remove");

    let mut processed_count = 0;
    let batch = 5000;

    // Iterate the machines hashmap
    for (_, machine) in machines.iter_mut() {
        // Check if machine has an ignored category from the ignored_categories list
        if machine.category.is_none() {
            // Remove machine from collection
            machines_to_remove.push(machine.name.clone());
            removed_machine_count += 1;
            continue;
        }
        let category = machine.category.as_ref().unwrap();
        if ignored_categories.contains(&category.as_str()) {
            // Remove machine from collection
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
