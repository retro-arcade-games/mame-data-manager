use csv::Writer;
use std::fs::File;
use crate::core::{data::MACHINES, models::Machine};

/**
 * Export the machines data to a CSV file
 */
pub fn export_to_csv(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the machines data
    let machines = MACHINES.lock().unwrap();
    let mut machines_vec: Vec<(&String, &Machine)> = machines.iter().collect();
    machines_vec.sort_by_key(|&(name, _)| name);

    // Machines file path
    let machines_path = format!("{}/machines.csv", export_path);

    let file = File::create(machines_path)?;
    let mut wtr = Writer::from_writer(file);

    wtr.write_record(&[
        "name",
        "source_file",
        "rom_of",
        "clone_of",
        "is_bios",
        "is_device",
        "runnable",
        "is_mechanical",
        "sample_of",
        "description",
        "year",
        "manufacturer",
        "driver_status",
        "languages",
        "players",
        "series",
        "category",
        "subcategory",
        "is_mature",
        "extended_name",
        "extended_manufacturer",
        "extended_players",
        "extended_is_parent",
        "extended_year",
    ])?;

    for (name, machine) in machines_vec {
        let extended_data = machine.extended_data.as_ref().unwrap();
        wtr.write_record(&[
            name,
            machine.source_file.as_deref().unwrap_or(""),
            machine.rom_of.as_deref().unwrap_or(""),
            machine.clone_of.as_deref().unwrap_or(""),
            machine
                .is_bios
                .map(|is_bios| if is_bios { "true" } else { "false" })
                .unwrap_or(""),
            machine
                .is_device
                .map(|is_device| if is_device { "true" } else { "false" })
                .unwrap_or(""),
            machine
                .runnable
                .map(|runnable| if runnable { "true" } else { "false" })
                .unwrap_or(""),
            machine
                .is_mechanical
                .map(|is_mechanical| if is_mechanical { "true" } else { "false" })
                .unwrap_or(""),
            machine.sample_of.as_deref().unwrap_or(""),
            machine.description.as_deref().unwrap_or(""),
            machine.year.as_deref().unwrap_or(""),
            machine.manufacturer.as_deref().unwrap_or(""),
            machine.driver_status.as_deref().unwrap_or(""),
            &machine.languages.join(", "),
            machine.players.as_deref().unwrap_or(""),
            machine.series.as_deref().unwrap_or(""),
            machine.category.as_deref().unwrap_or(""),
            machine.subcategory.as_deref().unwrap_or(""),
            machine
                .is_mature
                .map(|is_mature| if is_mature { "true" } else { "false" })
                .unwrap_or(""),
            extended_data.name.as_deref().unwrap_or(""),
            extended_data.manufacturer.as_deref().unwrap_or(""),
            extended_data.players.as_deref().unwrap_or(""),
            extended_data
                .is_parent
                .map(|is_parent| if is_parent { "true" } else { "false" })
                .unwrap_or(""),
            extended_data.year.as_deref().unwrap_or(""),
        ])?;
    }

    wtr.flush()?;

    Ok(())
}
