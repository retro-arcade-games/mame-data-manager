use crate::{
    core::{data::MACHINES, models::Machine},
    helpers::ui_helper::init_progress_bar,
};
use std::error::Error;

pub enum MachineFilter {
    Device,
    Bios,
    Mechanical,
    Modified,
    Clones,
    All,
}

/**
 * Remove machines that match the filter
 */
pub fn remove_machines_by_filter(machine_filter: MachineFilter) -> Result<u64, Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();

    // If the machines were not loaded, return an error
    if machines.is_empty() {
        return Err("No machines data loaded, please read the data first.".into());
    }

    let mut removed_machine_count: u64 = 0;
    let mut machines_to_remove: Vec<String> = Vec::new();

    let pb = init_progress_bar((machines.len() * 2) as u64, "finding machines to remove");

    let mut processed_count = 0;
    let batch = 5000;

    // Iterate the machines hashmap
    for (_, machine) in machines.iter_mut() {
        if filter_applies(machine, &machine_filter) {
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

/**
 * Check if the filter applies to the machine
 */
fn filter_applies(machine: &Machine, machine_filter: &MachineFilter) -> bool {
    match machine_filter {
        MachineFilter::Device => machine.is_device.unwrap_or(false),
        MachineFilter::Bios => machine.is_bios.unwrap_or(false),
        MachineFilter::Mechanical => machine.is_mechanical.unwrap_or(false),
        MachineFilter::Modified => {
            is_modified_machine(&machine.description.as_ref().unwrap_or(&"".to_string()))
                || has_invalid_manufacturer(machine)
                || has_invalid_players(&machine)
        }
        MachineFilter::Clones => is_clone(machine),
        MachineFilter::All => {
            machine.is_device.unwrap_or(false)
                || machine.is_bios.unwrap_or(false)
                || machine.is_mechanical.unwrap_or(false)
                || is_modified_machine(&machine.description.as_ref().unwrap_or(&"".to_string()))
                || has_invalid_manufacturer(machine)
                || has_invalid_players(&machine)
                || is_clone(machine)
        }
    }
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
 * Check if manufacturer is invalid
 */
fn has_invalid_manufacturer(machine: &Machine) -> bool {
    let invalid_manufacturers = vec!["unknown", "bootleg"];
    // Check if manufacturer in machine is invalid
    if let Some(manufacturer) = &machine.manufacturer {
        for invalid_manufacturer in invalid_manufacturers {
            if manufacturer
                .to_lowercase()
                .contains(&invalid_manufacturer.to_lowercase())
            {
                return true;
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

/**
 * Remove machines that are clones
 */
fn is_clone(machine: &Machine) -> bool {
    machine.clone_of.is_some() || machine.rom_of.is_some()
}
