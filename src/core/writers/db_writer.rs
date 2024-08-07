use rusqlite::{params, Connection, Result, Transaction};
use std::collections::HashSet;
use std::fs;

use crate::core::data::MACHINES;
use crate::core::models::Machine;
use crate::helpers::ui_helper::init_progress_bar;

/**
 * Create the database and the required tables.
 */
fn create_database(conn: &mut Connection) -> Result<()> {
    // Series table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS series (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    // Genres table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS genres (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    // Subgenres table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS subgenres (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL,
             genre_id INTEGER,
             UNIQUE(name, genre_id),
             FOREIGN KEY (genre_id) REFERENCES genres(id)
         )",
        [],
    )?;

    // Manufacturers table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS manufacturers (
             id INTEGER PRIMARY KEY AUTOINCREMENT,
             name TEXT NOT NULL UNIQUE
         )",
        [],
    )?;

    // Languages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Players table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // Machines table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS machines (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  name TEXT NOT NULL UNIQUE,
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
                  languages TEXT,
                  genre_id INTEGER,
                  subgenre_id INTEGER,
                  series_id INTEGER,
                  manufacturer_id INTEGER,
                  FOREIGN KEY (genre_id) REFERENCES genres(id),
                  FOREIGN KEY (subgenre_id) REFERENCES subgenres(id),
                  FOREIGN KEY (series_id) REFERENCES series(id)
                  FOREIGN KEY (manufacturer_id) REFERENCES manufacturers(id)
                  )",
        [],
    )?;

    // Machine languages table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS machine_languages (
            machine_id INTEGER,
            language_id INTEGER,
            FOREIGN KEY(machine_id) REFERENCES machines(id),
            FOREIGN KEY(language_id) REFERENCES languages(id),
            PRIMARY KEY(machine_id, language_id)
        )",
        [],
    )?;

    // Machine players table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS machine_players (
            machine_id INTEGER,
            player_id INTEGER,
            FOREIGN KEY(machine_id) REFERENCES machines(id),
            FOREIGN KEY(player_id) REFERENCES players(id),
            PRIMARY KEY(machine_id, player_id)
        )",
        [],
    )?;

    // Custom data table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS custom_datas (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  manufacturer TEXT,
                  players TEXT,
                  is_parent INTEGER,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // BIOS sets table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bios_sets (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  description TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // ROMs table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS roms (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  size INTEGER,
                  merge TEXT,
                  status TEXT,
                  crc TEXT,
                  sha1 TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // Device refs table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS device_refs (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // Softwares table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS softwares (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // Samples table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS samples (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // Disks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS disks (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  sha1 TEXT,
                  merge TEXT,
                  status TEXT,
                  region TEXT,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
                  )",
        [],
    )?;

    // History sections table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS history_sections (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  machine_name TEXT,
                  name TEXT,
                  text TEXT,
                  `order` INTEGER,
                  machine_id INTEGER,
                  FOREIGN KEY(machine_id) REFERENCES machines(id)
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
        "INSERT OR REPLACE INTO machines (
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
            "INSERT OR REPLACE INTO custom_datas (machine_name, name, manufacturer, players, is_parent) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![machine.name, custom_data.name, custom_data.manufacturer, custom_data.players, custom_data.is_parent],
        )?;
    }

    for bios_set in &machine.bios_sets {
        transaction.execute(
            "INSERT OR REPLACE INTO bios_sets (machine_name, name, description) VALUES (?1, ?2, ?3)",
            params![machine.name, bios_set.name, bios_set.description],
        )?;
    }

    for rom in &machine.roms {
        transaction.execute(
            "INSERT OR REPLACE INTO roms (
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
            "INSERT OR REPLACE INTO device_refs (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, device_ref.name],
        )?;
    }

    for software in &machine.software_list {
        transaction.execute(
            "INSERT OR REPLACE INTO softwares (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, software.name],
        )?;
    }

    for sample in &machine.samples {
        transaction.execute(
            "INSERT OR REPLACE INTO samples (machine_name, name) VALUES (?1, ?2)",
            params![machine.name, sample.name],
        )?;
    }

    for disk in &machine.disks {
        transaction.execute(
            "INSERT OR REPLACE INTO disks (
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
            "INSERT OR REPLACE INTO history_sections (
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
 * Extract and insert the languages from the machines data.
 */
fn extract_and_insert_languages(conn: &mut Connection) -> Result<()> {
    let unique_languages: HashSet<String> = {
        let mut stmt = conn.prepare("SELECT languages FROM machines")?;
        let machine_languages = stmt.query_map([], |row| {
            let languages: String = row.get(0)?;
            Ok(languages)
        })?;

        let mut unique_languages = HashSet::new();
        for lang_str in machine_languages {
            let languages = lang_str?;
            for language in languages.split(',').map(|s| s.trim()) {
                unique_languages.insert(language.to_string());
            }
        }
        unique_languages
    };

    let tx = conn.transaction()?;
    {
        let mut insert_stmt = tx.prepare("INSERT OR IGNORE INTO languages (name) VALUES (?)")?;
        for language in unique_languages {
            insert_stmt.execute([&language])?;
        }
    }
    tx.commit()?;

    Ok(())
}

/**
 * Insert the relationships between the machines and the languages.
 */
fn insert_machine_language_relationships(conn: &mut Connection) -> Result<()> {
    let machine_languages: Vec<(i64, String)> = {
        let mut stmt = conn.prepare("SELECT id, languages FROM machines")?;
        let machine_languages = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let languages: String = row.get(1)?;
            Ok((id, languages))
        })?;
        machine_languages.collect::<Result<Vec<_>, _>>()?
    };

    let tx = conn.transaction()?;
    {
        let mut insert_stmt = tx.prepare(
            "INSERT INTO machine_languages (machine_id, language_id)
             VALUES (?, (SELECT id FROM languages WHERE name = ?))",
        )?;
        for (machine_id, languages) in machine_languages {
            for language in languages.split(',').map(|s| s.trim()) {
                insert_stmt.execute(params![machine_id, language])?;
            }
        }
    }
    tx.commit()?;

    Ok(())
}

/**
 * Extract and insert the players from the custom data.
 */
fn extract_and_insert_players(conn: &mut Connection) -> Result<()> {
    let unique_players: HashSet<String> = {
        let mut stmt = conn.prepare("SELECT players FROM custom_datas")?;
        let custom_players = stmt.query_map([], |row| {
            let players: String = row.get(0)?;
            Ok(players)
        })?;

        let mut unique_players = HashSet::new();
        for players_str in custom_players {
            let players = players_str?;
            for player in players.split(',').map(|s| s.trim()) {
                unique_players.insert(player.to_string());
            }
        }
        unique_players
    };

    let tx = conn.transaction()?;
    {
        let mut insert_stmt = tx.prepare("INSERT OR IGNORE INTO players (name) VALUES (?)")?;
        for player in unique_players {
            insert_stmt.execute([&player])?;
        }
    }
    tx.commit()?;

    Ok(())
}

/**
 * Insert the relationships between the machines and the players.
 */
fn insert_machine_player_relationships(conn: &mut Connection) -> Result<()> {
    let machine_players: Vec<(i64, String)> = {
        let mut stmt = conn.prepare(
            "SELECT machines.id, custom_datas.players
             FROM machines
             JOIN custom_datas ON machines.id = custom_datas.machine_id",
        )?;
        let machine_players = stmt.query_map([], |row| {
            let machine_id: i64 = row.get(0)?;
            let players: String = row.get(1)?;
            Ok((machine_id, players))
        })?;
        machine_players.collect::<Result<Vec<_>, _>>()?
    };

    let tx = conn.transaction()?;
    {
        let mut insert_stmt = tx.prepare(
            "INSERT INTO machine_players (machine_id, player_id)
             VALUES (?, (SELECT id FROM players WHERE name = ?))",
        )?;
        for (machine_id, players) in machine_players {
            for player in players.split(',').map(|s| s.trim()) {
                insert_stmt.execute(params![machine_id, player])?;
            }
        }
    }
    tx.commit()?;

    Ok(())
}

/**
 * Create the relations between the machines and other entities.
 */
fn create_relations(conn: &Connection) -> Result<()> {
    // Add genres
    conn.execute(
        "INSERT OR IGNORE INTO genres (name)
         SELECT DISTINCT genre FROM machines WHERE genre IS NOT NULL",
        [],
    )?;
    // Update machines with genre_id
    conn.execute(
        "UPDATE machines
         SET genre_id = (SELECT id FROM genres WHERE genres.name = machines.genre)",
        [],
    )?;
    // Add subgenres (must be executed after updating machines with genre_id)
    conn.execute(
        "INSERT OR IGNORE INTO subgenres (name, genre_id)
         SELECT DISTINCT subgenre, genre_id
         FROM machines
         WHERE subgenre IS NOT NULL",
        [],
    )?;
    // Update machines with subgenre_id
    conn.execute(
        "UPDATE machines
         SET subgenre_id = (
             SELECT id
             FROM subgenres
             WHERE subgenres.name = machines.subgenre
               AND subgenres.genre_id = machines.genre_id
         )",
        [],
    )?;
    // Add series
    conn.execute(
        "INSERT OR IGNORE INTO series (name)
         SELECT DISTINCT series FROM machines WHERE series IS NOT NULL",
        [],
    )?;
    // Update machines with series_id
    conn.execute(
        "UPDATE machines
         SET series_id = (SELECT id FROM series WHERE series.name = machines.series)",
        [],
    )?;
    // Add manufacturers from custom data
    conn.execute(
        "INSERT OR IGNORE INTO manufacturers (name)
         SELECT DISTINCT manufacturer FROM custom_datas WHERE manufacturer IS NOT NULL",
        [],
    )?;
    // Update machines with manufacturer_id
    conn.execute(
        "UPDATE machines
         SET manufacturer_id = (
             SELECT manufacturers.id
             FROM manufacturers
             JOIN custom_datas ON custom_datas.manufacturer = manufacturers.name
             WHERE custom_datas.machine_name = machines.name
         )",
        [],
    )?;
    // Update custom data with machine_id
    conn.execute(
        "UPDATE custom_datas
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = custom_datas.machine_name
         )",
        [],
    )?;
    // Update bios sets with machine_id
    conn.execute(
        "UPDATE bios_sets
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = bios_sets.machine_name
         )",
        [],
    )?;
    // Update roms with machine_id
    conn.execute(
        "UPDATE roms
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = roms.machine_name
         )",
        [],
    )?;
    // Update device refs with machine_id
    conn.execute(
        "UPDATE device_refs
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = device_refs.machine_name
         )",
        [],
    )?;
    // Update softwares with machine_id
    conn.execute(
        "UPDATE softwares
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = softwares.machine_name
         )",
        [],
    )?;
    // Update samples with machine_id
    conn.execute(
        "UPDATE samples
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = samples.machine_name
         )",
        [],
    )?;
    // Update disks with machine_id
    conn.execute(
        "UPDATE disks
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = disks.machine_name
         )",
        [],
    )?;
    // Update history sections with machine_id
    conn.execute(
        "UPDATE history_sections
         SET machine_id = (
             SELECT id
             FROM machines
             WHERE machines.name = history_sections.machine_name
         )",
        [],
    )?;

    Ok(())
}

/**
 * Write the given machines data to the database.
 */
pub fn write_machines(db_path: &str) -> Result<()> {
    if fs::metadata(db_path).is_ok() {
        let _ = fs::remove_file(db_path);
    }

    let mut conn = Connection::open(db_path).unwrap();

    create_database(&mut conn)?;

    let machines = MACHINES.lock().unwrap();
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

    // Add relations
    create_relations(&mut conn)?;

    // Add languages relations
    extract_and_insert_languages(&mut conn)?;
    insert_machine_language_relationships(&mut conn)?;

    // Add players relations
    extract_and_insert_players(&mut conn)?;
    insert_machine_player_relationships(&mut conn)?;

    pb.finish_and_clear();

    Ok(())
}
