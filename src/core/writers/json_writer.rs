use crate::{
    core::{
        data::{CATEGORIES, LANGUAGES, MACHINES, MANUFACTURERS, PLAYERS, SERIES, SUBCATEGORIES},
        models::Machine,
    },
    helpers::ui_helper::init_progress_bar,
};
use serde_json::json;
use std::{collections::HashMap, error::Error, fs::File, io::Write, sync::MutexGuard};

/**
 * Export the machines data to a JSON file
 */
pub fn export_to_json(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    export_machines_to_json(export_path)?;

    // Export additional collections to separate JSON files
    let manufacturers = MANUFACTURERS.lock().unwrap();
    export_collection_to_json(manufacturers, export_path, "manufacturers")?;

    let series = SERIES.lock().unwrap();
    export_collection_to_json(series, export_path, "series")?;

    let languages = LANGUAGES.lock().unwrap();
    export_collection_to_json(languages, export_path, "languages")?;

    let players = PLAYERS.lock().unwrap();
    export_collection_to_json(players, export_path, "players")?;

    let categories = CATEGORIES.lock().unwrap();
    export_collection_to_json(categories, export_path, "categories")?;

    export_subcategories_to_json(export_path)?;

    Ok(())
}

/**
 * Export the machines data to a JSON file
 */
fn export_machines_to_json(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the machines data
    let machines = MACHINES.lock().unwrap();
    let mut machines_vec: Vec<(&String, &Machine)> = machines.iter().collect();
    machines_vec.sort_by_key(|&(name, _)| name);

    // Create the JSON writer for machines.json
    let mut machines_wtr = create_json_writer(export_path, "machines")?;

    // Collect all machines into a vector for JSON with transformations
    let machines_json: Vec<_> = machines_vec.into_iter().map(|(_, machine)| {
    json!({
        "name": machine.name,
        "source_file": machine.source_file,
        "rom_of": machine.rom_of,
        "clone_of": machine.clone_of,
        "is_bios": machine.is_bios,
        "is_device": machine.is_device,
        "runnable": machine.runnable,
        "is_mechanical": machine.is_mechanical,
        "sample_of": machine.sample_of,
        "description": machine.description,
        "year": machine.year,
        "manufacturer": machine.manufacturer,
        "bios_sets": machine.bios_sets.iter().map(|bs| json!({
            "name": bs.name,
            "description": bs.description,
        })).collect::<Vec<_>>(),
        "roms": machine.roms.iter().map(|rom| json!({
            "name": rom.name,
            "size": rom.size,
            "merge": rom.merge,
            "status": rom.status,
            "crc": rom.crc,
            "sha1": rom.sha1,
        })).collect::<Vec<_>>(),
        "device_refs": machine.device_refs.iter().map(|dr| dr.name.clone()).collect::<Vec<_>>(),
        "software_list": machine.software_list.iter().map(|sw| sw.name.clone()).collect::<Vec<_>>(),
        "samples": machine.samples.iter().map(|sample| sample.name.clone()).collect::<Vec<_>>(),
        "driver_status": machine.driver_status,
        "languages": machine.languages,
        "players": machine.players,
        "series": machine.series,
        "category": machine.category,
        "subcategory": machine.subcategory,
        "is_mature": machine.is_mature,
        "history_sections": machine.history_sections.iter().map(|hs| json!({
            "order": hs.order,
            "name": hs.name,
            "text": hs.text,
        })).collect::<Vec<_>>(),
        "disks": machine.disks.iter().map(|disk| json!({
            "name": disk.name,
            "sha1": disk.sha1,
            "merge": disk.merge,
            "status": disk.status,
            "region": disk.region,
        })).collect::<Vec<_>>(),
        "extended_data": machine.extended_data.as_ref().map(|ext| json!({
            "name": ext.name,
            "manufacturer": ext.manufacturer,
            "players": ext.players.as_deref().unwrap_or("")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>(),
            "is_parent": ext.is_parent,
            "year": ext.year,
        })),
        "resources": machine.resources.iter().map(|res| json!({
            "type_": res.type_,
            "name": res.name,
            "size": res.size,
            "crc": res.crc,
            "sha1": res.sha1,
        })).collect::<Vec<_>>(),
    })
}).collect::<Vec<_>>();

    let mut processed_count = 0;
    let batch = 50;

    let total_elements = machines_json.len();
    let pb = init_progress_bar(total_elements as u64, "machines");

    // Write the opening of the array
    machines_wtr.write_all(b"[\n")?;

    for (i, item) in machines_json.iter().enumerate() {
        if i > 0 {
            machines_wtr.write_all(b",\n")?;
        }
        serde_json::to_writer_pretty(&mut machines_wtr, &item)?;

        processed_count += 1;
        if processed_count % batch == 0 {
            pb.inc(batch);
        }
    }

    // Write the closing of the array
    machines_wtr.write_all(b"\n]")?;

    machines_wtr.flush()?;

    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();

    Ok(())
}

/**
 * Create a JSON writer
 */
fn create_json_writer(export_path: &str, file_name: &str) -> Result<File, Box<dyn Error>> {
    let file_path = format!("{}/{}.json", export_path, file_name);
    let file = File::create(file_path)?;
    Ok(file)
}

/**
 * Export the collection data to a JSON file
 */
fn export_collection_to_json(
    data: MutexGuard<HashMap<String, usize>>,
    export_path: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut data_vec: Vec<(&String, &usize)> = data.iter().collect();
    data_vec.sort_by_key(|&(name, _)| name);

    let mut wtr = create_json_writer(export_path, file_name)?;

    // Convert the data to a vector of JSON objects
    let json_data: Vec<_> = data_vec
        .into_iter()
        .map(|(name, machines)| {
            json!({
                "name": name,
                "machines": machines,
            })
        })
        .collect();

    // Write the data
    serde_json::to_writer_pretty(&mut wtr, &json_data)?;
    wtr.flush()?;

    Ok(())
}

/**
 * Export the subcategories data to a JSON file
 */
fn export_subcategories_to_json(export_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let subcategories = SUBCATEGORIES.lock().unwrap();
    let mut subcategories_vec: Vec<(&String, &usize)> = subcategories.iter().collect();
    subcategories_vec.sort_by_key(|&(name, _)| name);

    let mut subcategories_wtr = create_json_writer(export_path, "subcategories")?;

    let json_data: Vec<_> = subcategories_vec
        .into_iter()
        .map(|(name, machines)| {
            let splitted: Vec<&str> = name.split(" - ").collect();
            let category = splitted[0];
            let subcategory = splitted[1];
            json!({
                "category": category,
                "subcategory": subcategory,
                "machines": machines,
            })
        })
        .collect();

    // Write the subcategories data
    serde_json::to_writer_pretty(&mut subcategories_wtr, &json_data)?;
    subcategories_wtr.flush()?;

    Ok(())
}
