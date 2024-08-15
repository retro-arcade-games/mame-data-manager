use crate::core::{
    data::{CATEGORIES, LANGUAGES, MACHINES, MANUFACTURERS, PLAYERS, SERIES, SUBCATEGORIES},
    models::Machine,
};
use csv::Writer;
use std::{collections::HashMap, error::Error, fs::File, sync::MutexGuard};

/**
 * Export the machines data to a CSV file
 */
pub fn export_to_csv(export_path: &str) -> Result<(), Box<dyn Error>> {
    // Get the machines data
    let machines = MACHINES.lock().unwrap();

    // If the machines were not loaded, return an error
    if machines.is_empty() {
        return Err("No machines data loaded, please read the data first.".into());
    }

    let mut machines_vec: Vec<(&String, &Machine)> = machines.iter().collect();
    machines_vec.sort_by_key(|&(name, _)| name);

    // Create the CSV writers
    let mut machines_wtr = create_writer(export_path, "machines")?;
    let mut roms_wtr = create_writer(export_path, "roms")?;
    let mut bios_sets_wtr = create_writer(export_path, "bios_sets")?;
    let mut device_refs_wtr = create_writer(export_path, "device_refs")?;
    let mut disks_wtr = create_writer(export_path, "disks")?;
    let mut softwares_wtr = create_writer(export_path, "softwares")?;
    let mut samples_wtr = create_writer(export_path, "samples")?;
    let mut history_sections_wtr = create_writer(export_path, "history_sections")?;
    let mut resources_wtr = create_writer(export_path, "resources")?;

    // Write the CSV headers
    write_csv_header(
        &mut machines_wtr,
        &[
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
        ],
    )?;
    write_csv_header(
        &mut roms_wtr,
        &[
            "machine_name",
            "name",
            "size",
            "merge",
            "status",
            "crc",
            "sha1",
        ],
    )?;
    write_csv_header(&mut bios_sets_wtr, &["machine_name", "name", "description"])?;
    write_csv_header(&mut device_refs_wtr, &["machine_name", "name"])?;
    write_csv_header(
        &mut disks_wtr,
        &["machine_name", "name", "sha1", "merge", "status", "region"],
    )?;
    write_csv_header(&mut softwares_wtr, &["machine_name", "name"])?;
    write_csv_header(&mut samples_wtr, &["machine_name", "name"])?;
    write_csv_header(
        &mut history_sections_wtr,
        &["machine_name", "name", "text", "order"],
    )?;
    write_csv_header(
        &mut resources_wtr,
        &["machine_name", "type", "name", "size", "crc", "sha1"],
    )?;

    for (name, machine) in machines_vec {
        // Write machine
        write_csv_record(
            &mut machines_wtr,
            &[
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
                machine
                    .extended_data
                    .as_ref()
                    .unwrap()
                    .name
                    .as_deref()
                    .unwrap_or(""),
                machine
                    .extended_data
                    .as_ref()
                    .unwrap()
                    .manufacturer
                    .as_deref()
                    .unwrap_or(""),
                machine
                    .extended_data
                    .as_ref()
                    .unwrap()
                    .players
                    .as_deref()
                    .unwrap_or(""),
                machine
                    .extended_data
                    .as_ref()
                    .unwrap()
                    .is_parent
                    .map(|is_parent| if is_parent { "true" } else { "false" })
                    .unwrap_or(""),
                machine
                    .extended_data
                    .as_ref()
                    .unwrap()
                    .year
                    .as_deref()
                    .unwrap_or(""),
            ],
        )?;
        // Write roms
        for rom in &machine.roms {
            write_csv_record(
                &mut roms_wtr,
                &[
                    name,
                    &rom.name,
                    &rom.size.to_string(),
                    rom.merge.as_deref().unwrap_or(""),
                    rom.status.as_deref().unwrap_or(""),
                    rom.crc.as_deref().unwrap_or(""),
                    rom.sha1.as_deref().unwrap_or(""),
                ],
            )?;
        }
        // Write bios sets
        for bios_set in &machine.bios_sets {
            write_csv_record(
                &mut bios_sets_wtr,
                &[name, &bios_set.name, &bios_set.description],
            )?;
        }
        // Write device refs
        for device_ref in &machine.device_refs {
            write_csv_record(&mut device_refs_wtr, &[name, &device_ref.name])?;
        }
        // Write disks
        for disk in &machine.disks {
            write_csv_record(
                &mut disks_wtr,
                &[
                    name,
                    &disk.name,
                    disk.sha1.as_deref().unwrap_or(""),
                    disk.merge.as_deref().unwrap_or(""),
                    disk.status.as_deref().unwrap_or(""),
                    disk.region.as_deref().unwrap_or(""),
                ],
            )?;
        }
        // Write softwares
        for software in &machine.software_list {
            write_csv_record(&mut softwares_wtr, &[name, &software.name])?;
        }
        // Write samples
        for sample in &machine.samples {
            write_csv_record(&mut samples_wtr, &[name, &sample.name])?;
        }
        // Write history sections
        for history_section in &machine.history_sections {
            write_csv_record(
                &mut history_sections_wtr,
                &[
                    name,
                    &history_section.name,
                    &history_section.text,
                    &history_section.order.to_string(),
                ],
            )?;
        }
        // Write resources
        for resource in &machine.resources {
            write_csv_record(
                &mut resources_wtr,
                &[
                    name,
                    &resource.type_,
                    &resource.name,
                    &resource.size.to_string(),
                    &resource.crc,
                    &resource.sha1,
                ],
            )?;
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

    let manufacturers = MANUFACTURERS.lock().unwrap();
    export_collection(
        manufacturers,
        export_path,
        "manufacturers",
        &["name", "machines"],
    )?;

    let series = SERIES.lock().unwrap();
    export_collection(series, export_path, "series", &["name", "machines"])?;

    let languages = LANGUAGES.lock().unwrap();
    export_collection(languages, export_path, "languages", &["name", "machines"])?;

    let players = PLAYERS.lock().unwrap();
    export_collection(players, export_path, "players", &["name", "machines"])?;

    let categories = CATEGORIES.lock().unwrap();
    export_collection(categories, export_path, "categories", &["name", "machines"])?;

    export_subcategories(export_path)?;

    Ok(())
}

/**
 * Create a CSV writer
 */
fn create_writer(export_path: &str, file_name: &str) -> Result<Writer<File>, Box<dyn Error>> {
    let file_path = format!("{}/{}.csv", export_path, file_name);
    let file = File::create(file_path)?;
    let writer = Writer::from_writer(file);
    Ok(writer)
}

/**
 * Write the CSV header
 */
fn write_csv_header(wtr: &mut Writer<File>, headers: &[&str]) -> Result<(), csv::Error> {
    wtr.write_record(headers)
}

/**
 * Write the CSV record
 */
fn write_csv_record<W: std::io::Write>(
    wtr: &mut Writer<W>,
    fields: &[&str],
) -> Result<(), csv::Error> {
    wtr.write_record(fields)
}

/**
 * Export the collection data to a CSV file
 */
fn export_collection(
    data: MutexGuard<HashMap<String, usize>>,
    export_path: &str,
    file_name: &str,
    headers: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data_vec: Vec<(&String, &usize)> = data.iter().collect();
    data_vec.sort_by_key(|&(name, _)| name);

    // Create the file path
    let file_path = format!("{}/{}.csv", export_path, file_name);
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    // Write the header
    wtr.write_record(headers)?;

    // Write the data
    for (name, machines) in data_vec {
        wtr.write_record(&[name, &machines.to_string()])?;
    }

    wtr.flush()?;

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
        subcategories_wtr.write_record(&[category, subcategory, &machines.to_string()])?;
    }

    subcategories_wtr.flush()?;

    Ok(())
}
