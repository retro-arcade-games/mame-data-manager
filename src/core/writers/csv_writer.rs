use crate::core::{
    data::{CATEGORIES, LANGUAGES, MACHINES, MANUFACTURERS, PLAYERS, SERIES, SUBCATEGORIES},
    models::Machine,
};
use csv::Writer;
use std::fs::File;

/**
 * Export the machines data to a CSV file
 */
pub fn export_to_csv(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the machines data
    let machines = MACHINES.lock().unwrap();
    let mut machines_vec: Vec<(&String, &Machine)> = machines.iter().collect();
    machines_vec.sort_by_key(|&(name, _)| name);

    // Machines writer
    let machines_path = format!("{}/machines.csv", export_path);
    let machines_file = File::create(machines_path)?;
    let mut machines_wtr = Writer::from_writer(machines_file);

    // Roms writer
    let roms_path = format!("{}/roms.csv", export_path);
    let roms_file = File::create(roms_path)?;
    let mut roms_wtr = Writer::from_writer(roms_file);

    // Bios sets writer
    let bios_sets_path = format!("{}/bios_sets.csv", export_path);
    let bios_sets_file = File::create(bios_sets_path)?;
    let mut bios_sets_wtr = Writer::from_writer(bios_sets_file);

    // Device refs writer
    let device_refs_path = format!("{}/device_refs.csv", export_path);
    let device_refs_file = File::create(device_refs_path)?;
    let mut device_refs_wtr = Writer::from_writer(device_refs_file);

    // Disks writer
    let disks_path = format!("{}/disks.csv", export_path);
    let disks_file = File::create(disks_path)?;
    let mut disks_wtr = Writer::from_writer(disks_file);

    // Softwares writer
    let softwares_path = format!("{}/softwares.csv", export_path);
    let softwares_file = File::create(softwares_path)?;
    let mut softwares_wtr = Writer::from_writer(softwares_file);

    // Samples writer
    let samples_path = format!("{}/samples.csv", export_path);
    let samples_file = File::create(samples_path)?;
    let mut samples_wtr = Writer::from_writer(samples_file);

    // History sections writer
    let history_sections_path = format!("{}/history_sections.csv", export_path);
    let history_sections_file = File::create(history_sections_path)?;
    let mut history_sections_wtr = Writer::from_writer(history_sections_file);

    // Resources writer
    let resources_path = format!("{}/resources.csv", export_path);
    let resources_file = File::create(resources_path)?;
    let mut resources_wtr = Writer::from_writer(resources_file);

    write_machines_header(&mut machines_wtr)?;
    write_roms_header(&mut roms_wtr)?;
    write_bios_sets_header(&mut bios_sets_wtr)?;
    write_device_refs_header(&mut device_refs_wtr)?;
    write_disks_header(&mut disks_wtr)?;
    write_softwares_header(&mut softwares_wtr)?;
    write_samples_header(&mut samples_wtr)?;
    write_history_sections_header(&mut history_sections_wtr)?;
    write_resources_header(&mut resources_wtr)?;

    for (name, machine) in machines_vec {
        write_machine_record(&mut machines_wtr, name, machine)?;
        // Write roms
        for rom in &machine.roms {
            write_rom_record(&mut roms_wtr, name, rom)?;
        }
        // Write bios sets
        for bios_set in &machine.bios_sets {
            write_bios_set_record(&mut bios_sets_wtr, name, bios_set)?;
        }
        // Write device refs
        for device_ref in &machine.device_refs {
            write_device_ref_record(&mut device_refs_wtr, name, device_ref)?;
        }
        // Write disks
        for disk in &machine.disks {
            write_disk_record(&mut disks_wtr, name, disk)?;
        }
        // Write softwares
        for software in &machine.software_list {
            write_software_record(&mut softwares_wtr, name, software)?;
        }
        // Write samples
        for sample in &machine.samples {
            write_sample_record(&mut samples_wtr, name, sample)?;
        }
        // Write history sections
        for history_section in &machine.history_sections {
            write_history_section_record(&mut history_sections_wtr, name, history_section)?;
        }
        // Write resources
        for resource in &machine.resources {
            write_resource_record(&mut resources_wtr, name, resource)?;
        }
    }

    machines_wtr.flush()?;
    roms_wtr.flush()?;
    bios_sets_wtr.flush()?;
    device_refs_wtr.flush()?;
    disks_wtr.flush()?;
    softwares_wtr.flush()?;
    samples_wtr.flush()?;
    history_sections_wtr.flush()?;
    resources_wtr.flush()?;

    export_manufacturers(export_path)?;
    export_series(export_path)?;
    export_languages(export_path)?;
    export_players(export_path)?;
    export_categories(export_path)?;
    export_subcategories(export_path)?;

    Ok(())
}

/**
 * Export the manufacturers data to a CSV file
 */
fn export_manufacturers(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manufacturers = MANUFACTURERS.lock().unwrap();
    let mut manufacturers_vec: Vec<(&String, &usize)> = manufacturers.iter().collect();
    manufacturers_vec.sort_by_key(|&(name, _)| name);

    // Manufacturers writer
    let manufacturers_path = format!("{}/manufacturers.csv", export_path);
    let manufacturers_file = File::create(manufacturers_path)?;
    let mut manufacturers_wtr = Writer::from_writer(manufacturers_file);

    // Write the manufacturers header
    manufacturers_wtr.write_record(&["name", "machines"])?;

    for (name, machines) in manufacturers_vec {
        manufacturers_wtr.write_record(&[name, &machines.to_string()])?;
    }

    manufacturers_wtr.flush()?;

    Ok(())
}

/**
 * Export the series data to a CSV file
 */
fn export_series(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let series = SERIES.lock().unwrap();
    let mut series_vec: Vec<(&String, &usize)> = series.iter().collect();
    series_vec.sort_by_key(|&(name, _)| name);

    // Series writer
    let series_path = format!("{}/series.csv", export_path);
    let series_file = File::create(series_path)?;
    let mut series_wtr = Writer::from_writer(series_file);

    // Write the series header
    series_wtr.write_record(&["name", "machines"])?;

    for (name, machines) in series_vec {
        series_wtr.write_record(&[name, &machines.to_string()])?;
    }

    series_wtr.flush()?;

    Ok(())
}

fn export_languages(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let languages = LANGUAGES.lock().unwrap();
    let mut languages_vec: Vec<(&String, &usize)> = languages.iter().collect();
    languages_vec.sort_by_key(|&(name, _)| name);

    // Languages writer
    let languages_path = format!("{}/languages.csv", export_path);
    let languages_file = File::create(languages_path)?;
    let mut languages_wtr = Writer::from_writer(languages_file);

    // Write the languages header
    languages_wtr.write_record(&["name", "machines"])?;

    for (name, machines) in languages_vec {
        languages_wtr.write_record(&[name, &machines.to_string()])?;
    }

    languages_wtr.flush()?;

    Ok(())
}

/**
 * Export the players data to a CSV file
 */
fn export_players(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let players = PLAYERS.lock().unwrap();
    let mut players_vec: Vec<(&String, &usize)> = players.iter().collect();
    players_vec.sort_by_key(|&(name, _)| name);

    // Players writer
    let players_path = format!("{}/players.csv", export_path);
    let players_file = File::create(players_path)?;
    let mut players_wtr = Writer::from_writer(players_file);

    // Write the players header
    players_wtr.write_record(&["name", "machines"])?;

    for (name, machines) in players_vec {
        players_wtr.write_record(&[name, &machines.to_string()])?;
    }

    players_wtr.flush()?;

    Ok(())
}

/**
 * Export the categories data to a CSV file
 */
fn export_categories(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let categories = CATEGORIES.lock().unwrap();
    let mut categories_vec: Vec<(&String, &usize)> = categories.iter().collect();
    categories_vec.sort_by_key(|&(name, _)| name);

    // Categories writer
    let categories_path = format!("{}/categories.csv", export_path);
    let categories_file = File::create(categories_path)?;
    let mut categories_wtr = Writer::from_writer(categories_file);

    // Write the categories header
    categories_wtr.write_record(&["name", "machines"])?;

    for (name, machines) in categories_vec {
        categories_wtr.write_record(&[name, &machines.to_string()])?;
    }

    categories_wtr.flush()?;

    Ok(())
}

/**
 * Export the subcategories data to a CSV file
 */
fn export_subcategories(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subcategories = SUBCATEGORIES.lock().unwrap();
    let mut subcategories_vec: Vec<(&String, &usize)> = subcategories.iter().collect();
    subcategories_vec.sort_by_key(|&(name, _)| name);

    // Subcategories writer
    let subcategories_path = format!("{}/subcategories.csv", export_path);
    let subcategories_file = File::create(subcategories_path)?;
    let mut subcategories_wtr = Writer::from_writer(subcategories_file);

    // Write the subcategories header
    subcategories_wtr.write_record(&["category", "subcategory", "machines"])?;

    for (name, machines) in subcategories_vec {
        let splitted: Vec<&str> = name.split(" - ").collect();
        let category = splitted[0];
        let subcategory = splitted[1];
        subcategories_wtr.write_record(&[category,subcategory, &machines.to_string()])?;
    }

    subcategories_wtr.flush()?;

    Ok(())
}

/**
 * Write the machines header to the CSV file
 */
fn write_machines_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
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
    ])
}

/**
 * Write the machine record to the CSV file
 */
fn write_machine_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    name: &str,
    machine: &Machine,
) -> Result<(), csv::Error> {
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
    ])
}

/**
 * Write the roms header to the CSV file
 */
fn write_roms_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&[
        "machine_name",
        "name",
        "size",
        "merge",
        "status",
        "crc",
        "sha1",
    ])
}

/**
 * Write the rom record to the CSV file
 */
fn write_rom_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    rom: &crate::core::models::Rom,
) -> Result<(), csv::Error> {
    wtr.write_record(&[
        machine_name,
        &rom.name,
        &rom.size.to_string(),
        rom.merge.as_deref().unwrap_or(""),
        rom.status.as_deref().unwrap_or(""),
        rom.crc.as_deref().unwrap_or(""),
        rom.sha1.as_deref().unwrap_or(""),
    ])
}

/**
 * Write the bios sets header to the CSV file
 */
fn write_bios_sets_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name", "description"])
}

/**
 * Write the bios set record to the CSV file
 */
fn write_bios_set_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    bios_set: &crate::core::models::BiosSet,
) -> Result<(), csv::Error> {
    wtr.write_record(&[machine_name, &bios_set.name, &bios_set.description])
}

/**
 * Write the device refs header to the CSV file
 */
fn write_device_refs_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name"])
}

/**
 * Write the device ref record to the CSV file
 */
fn write_device_ref_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    device_ref: &crate::core::models::DeviceRef,
) -> Result<(), csv::Error> {
    wtr.write_record(&[machine_name, &device_ref.name])
}

/**
 * Write the disks header to the CSV file
 */
fn write_disks_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name", "sha1", "merge", "status", "region"])
}

/**
 * Write the disk record to the CSV file
 */
fn write_disk_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    disk: &crate::core::models::Disk,
) -> Result<(), csv::Error> {
    wtr.write_record(&[
        machine_name,
        &disk.name,
        disk.sha1.as_deref().unwrap_or(""),
        disk.merge.as_deref().unwrap_or(""),
        disk.status.as_deref().unwrap_or(""),
        disk.region.as_deref().unwrap_or(""),
    ])
}

/**
 * Write the softwares header to the CSV file
 */
fn write_softwares_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name"])
}

/**
 * Write the software record to the CSV file
 */
fn write_software_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    software: &crate::core::models::Software,
) -> Result<(), csv::Error> {
    wtr.write_record(&[machine_name, &software.name])
}

/**
 * Write the samples header to the CSV file
 */
fn write_samples_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name"])
}

/**
 * Write the sample record to the CSV file
 */
fn write_sample_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    sample: &crate::core::models::Sample,
) -> Result<(), csv::Error> {
    wtr.write_record(&[machine_name, &sample.name])
}

/**
 * Write the history sections header to the CSV file
 */
fn write_history_sections_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "name", "text", "order"])
}

/**
 * Write the history section record to the CSV file
 */
fn write_history_section_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    history_section: &crate::core::models::HistorySection,
) -> Result<(), csv::Error> {
    wtr.write_record(&[
        machine_name,
        &history_section.name,
        &history_section.text,
        &history_section.order.to_string(),
    ])
}

/**
 * Write the resources header to the CSV file
 */
fn write_resources_header(wtr: &mut Writer<File>) -> Result<(), csv::Error> {
    wtr.write_record(&["machine_name", "type", "name", "size", "crc", "sha1"])
}

/**
 * Write the resource record to the CSV file
 */
fn write_resource_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    machine_name: &str,
    resource: &crate::core::models::Resource,
) -> Result<(), csv::Error> {
    wtr.write_record(&[
        machine_name,
        &resource.type_,
        &resource.name,
        &resource.size.to_string(),
        &resource.crc,
        &resource.sha1,
    ])
}
