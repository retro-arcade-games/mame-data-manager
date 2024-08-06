use rusqlite::{params, Connection, Result, Transaction};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use crate::core::models::Machine;
use crate::helpers::ui_helper::init_progress_bar;

/**
 * Create the database and the required tables.
 */
fn create_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS machine (
                  name TEXT PRIMARY KEY,
                  source_file TEXT,
                  rom_of TEXT,
                  clone_of TEXT,
                  is_bios INTEGER,
                  is_device INTEGER,
                  runnable INTEGER,
                  is_mechanical INTEGER,
                  sample_of TEXT,
                  description TEXT,
                  year TEXT,
                  manufacturer TEXT,
                  driver_status TEXT,
                  players TEXT,
                  series TEXT,
                  genre TEXT,
                  subgenre TEXT,
                  is_mature INTEGER,
                  languages TEXT
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS custom_data (
                  machine_name TEXT,
                  name TEXT,
                  manufacturer TEXT,
                  players TEXT,
                  is_parent INTEGER,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS bios_set (
                  machine_name TEXT,
                  name TEXT,
                  description TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS rom (
                  machine_name TEXT,
                  name TEXT,
                  size INTEGER,
                  merge TEXT,
                  status TEXT,
                  crc TEXT,
                  sha1 TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS device_ref (
                  machine_name TEXT,
                  name TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS software (
                  machine_name TEXT,
                  name TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sample (
                  machine_name TEXT,
                  name TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS disk (
                  machine_name TEXT,
                  name TEXT,
                  sha1 TEXT,
                  merge TEXT,
                  status TEXT,
                  region TEXT,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history_section (
                  machine_name TEXT,
                  name TEXT,
                  text TEXT,
                  `order` INTEGER,
                  FOREIGN KEY(machine_name) REFERENCES machine(name)
                  )",
        [],
    )?;

    Ok(())
}

/**
 * Insert the given machine data into the database.
 */
fn insert_machine_data(transaction: &Transaction, machine: &Machine) -> Result<()> {
    transaction.execute(
        "INSERT OR REPLACE INTO machine (
                  name, source_file, rom_of, clone_of, is_bios, is_device, runnable, is_mechanical, sample_of,
                  description, year, manufacturer, driver_status, players, series, genre, subgenre, is_mature, languages
                  ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            machine.name,
            machine.source_file,
            machine.rom_of,
            machine.clone_of,
            machine.is_bios,
            machine.is_device,
            machine.runnable,
            machine.is_mechanical,
            machine.sample_of,
            machine.description,
            machine.year,
            machine.manufacturer,
            machine.driver_status,
            machine.players,
            machine.series,
            machine.genre,
            machine.subgenre,
            machine.is_mature,
            machine.languages.join(", ")
        ],
    )?;

    if let Some(custom_data) = &machine.custom_data {
        transaction.execute(
            "INSERT OR REPLACE INTO custom_data (machine_name, name, manufacturer, players, is_parent) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![machine.name, custom_data.name, custom_data.manufacturer, custom_data.players, custom_data.is_parent],
        )?;
    }

    for bios_set in &machine.bios_sets {
        transaction.execute(
            "INSERT OR REPLACE INTO bios_set (machine_name, name, description) VALUES (?1, ?2, ?3)",
            params![machine.name, bios_set.name, bios_set.description],
        )?;
    }

    for rom in &machine.roms {
        transaction.execute(
            "INSERT OR REPLACE INTO rom (
                      machine_name, name, size, merge, status, crc, sha1
                      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                machine.name,
                rom.name,
                rom.size,
                rom.merge,
                rom.status,
                rom.crc,
                rom.sha1
            ],
        )?;
    }

    for device_ref in &machine.device_refs {
        transaction.execute(
            "INSERT OR REPLACE INTO device_ref (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, device_ref.name],
        )?;
    }

    for software in &machine.software_list {
        transaction.execute(
            "INSERT OR REPLACE INTO software (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, software.name],
        )?;
    }

    for sample in &machine.samples {
        transaction.execute(
            "INSERT OR REPLACE INTO sample (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, sample.name],
        )?;
    }

    for disk in &machine.disks {
        transaction.execute(
            "INSERT OR REPLACE INTO disk (
                      machine_name, name, sha1, merge, status, region
                      ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                machine.name,
                disk.name,
                disk.sha1,
                disk.merge,
                disk.status,
                disk.region
            ],
        )?;
    }

    for history_section in &machine.history_sections {
        transaction.execute(
            "INSERT OR REPLACE INTO history_section (
                      machine_name, name, text, `order`
                      ) VALUES (?1, ?2, ?3, ?4)",
            params![
                machine.name,
                history_section.name,
                history_section.text,
                history_section.order
            ],
        )?;
    }

    Ok(())
}

/**
 * Write the given machines data to the database.
 */
pub fn write_machines(db_path: &str, machines: Arc<Mutex<HashMap<String, Machine>>>) -> Result<()> {
    if fs::metadata(db_path).is_ok() {
        let _ = fs::remove_file(db_path);
    }

    let mut conn = Connection::open(db_path).unwrap();

    create_database(&conn)?;

    let machines = machines.lock().unwrap();
    let batch_size = 5000;
    let mut batch_count = 0;

    let total_elements = machines.len();
    let pb = init_progress_bar(total_elements as u64, "machines");

    let mut processed_count = 0;
    let batch = 5000;

    let mut transaction = conn.transaction()?;
    for machine in machines.values() {
        insert_machine_data(&transaction, machine)?;

        batch_count += 1;
        if batch_count >= batch_size {
            transaction.commit()?;
            transaction = conn.transaction()?;
            batch_count = 0;
        }

        processed_count += 1;
        if processed_count % batch == 0 {
            pb.inc(batch);
        }
    }

    // Commit any remaining transactions
    transaction.commit()?;

    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();

    Ok(())
}
