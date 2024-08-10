mod core;
mod helpers;
mod modules;

use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::fs_helper::{check_folder_structure, find_file_with_pattern, get_file_name, PATHS};
use modules::data_filtering;
use num_format::{Locale, ToFormattedString};

use core::data::MACHINES;
use core::data_types::{DataType, DATA_TYPES};
use core::filters::{
    filter_genres, filter_non_games, manufacturer_refactor, name_refactor, nplayers_refactor,
};
use core::models::Machine;
use core::writers::{db_writer, json_writer};
use helpers::data_source_helper::get_data_source;
use helpers::file_download_helper::download_file;
use helpers::file_extractor_helper::extract_file;
use helpers::ui_helper::{
    icons::*, print_step_message, println_step_message, show_splash_screen, show_title,
};
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() -> Result<(), Box<dyn Error>> {
    check_folder_structure()?;
    show_menu()?;
    Ok(())
}

/**
 * Show the main menu.
 */
fn show_menu() -> Result<(), Box<dyn Error>> {
    show_splash_screen();

    loop {
        show_title();

        let selections = &[
            "Download files",
            "Extract files",
            "Read files",
            "View stats",
            "Cleanup & filter data...",
            "Export data...",
            "Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => download_files()?,
            1 => extract_files()?,
            2 => read_files()?,
            3 => show_stats()?,
            4 => data_filtering::show_filtering_submenu()?,
            5 => show_export_submenu()?,
            6 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Show the export submenu.
 */
fn show_export_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &["Export to SQLite", "Export to JSON", "Back"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => create_database()?,
            1 => create_json()?,
            2 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Download the files from the data sources.
 */
fn download_files() -> Result<(), Box<dyn Error>> {
    let mut count = 0;

    for data_type in DATA_TYPES.iter() {
        count += 1;
        download_data_file(data_type, count)?;
        println!("")
    }

    Ok(())
}

/**
 * Download the data file.
 */
fn download_data_file(data_type: &DataType, count: usize) -> Result<(), Box<dyn Error>> {
    let message = format!("Getting URL for {}...", data_type.name);
    print_step_message(&message, count, DATA_TYPES.len(), DOWNLOAD);

    if let Ok(source_url) = get_data_source(data_type.source, data_type.source_match) {
        let file_name = get_file_name(&source_url);
        let file_path = format!("{}{}", PATHS.download_path, file_name);

        if !Path::new(&file_path).exists() {
            let message = format!(
                "Downloading {} from {}",
                style(file_name.clone()).cyan(),
                data_type.source
            );
            print_step_message(&message, count, DATA_TYPES.len(), DOWNLOAD);

            download_file(&source_url, &file_path)?;

            let message = format!("{} downloaded successfully", style(file_name).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), SUCCESS);
        } else {
            let message = format!("{} already exists (skipped)", style(file_name).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), INFO);
        }
    } else {
        let message = format!("Failed getting matching source for {}", data_type.name);
        print_step_message(&message, count, DATA_TYPES.len(), ERROR);
    }
    Ok(())
}

/**
 * Extract the downloaded files.
 */
fn extract_files() -> Result<(), Box<dyn Error>> {
    let mut count = 0;

    for data_type in DATA_TYPES.iter() {
        count += 1;

        let extracted_folder = format!("{}{}", PATHS.extract_path, data_type.name.to_lowercase());

        let message = format!("Checking if {} file already extracted...", data_type.name);
        println_step_message(&message, count, DATA_TYPES.len(), LOUPE);

        // Check if the file already exists in the extracted folder
        if let Some(existing_file_path) =
            find_file_with_pattern(&extracted_folder, &data_type.data_file_pattern)
        {
            let data_file = existing_file_path.split('/').last().unwrap();

            let message = format!("{} already exists (skipped)", style(data_file).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), INFO);

            continue;
        }

        let mut continue_extraction = false;
        let mut file_name: Option<String> = None;

        // If zip file is not present then download it and extract it
        while !continue_extraction {
            file_name =
                match find_file_with_pattern(&PATHS.download_path, &data_type.zip_file_pattern) {
                    Some(path) => {
                        let file_name = path.split('/').last().unwrap().to_owned();
                        continue_extraction = true;
                        Some(file_name)
                    }
                    None => {
                        download_data_file(data_type, count)?;
                        None
                    }
                };
        }

        let file_name = file_name.unwrap();

        // Get the file path
        let file_path = format!("{}{}", PATHS.download_path, file_name);

        // Check if the file exists
        if Path::new(&file_path).exists() {
            let message = format!(
                "Extracting {} to {}",
                style(file_name.clone()).cyan(),
                extracted_folder
            );
            print_step_message(&message, count, DATA_TYPES.len(), FOLDER);

            extract_file(&file_path, &extracted_folder)?;

            let message = format!("{} extracted successfully", style(file_name).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), SUCCESS);
        } else {
            let message = format!("File for {} not found", data_type.name);
            print_step_message(&message, count, DATA_TYPES.len(), ERROR);
        }
    }
    Ok(())
}

/**
 * Read the extracted files.
 */
fn read_files() -> Result<(), Box<dyn Error>> {
    let mut count = 0;

    for data_type in DATA_TYPES.iter() {
        count += 1;

        let extracted_folder = format!("{}{}", PATHS.extract_path, data_type.name.to_lowercase());

        let message = format!("Checking if {} file exists...", data_type.name);
        println_step_message(&message, count, DATA_TYPES.len(), LOUPE);

        if let Some(data_file_path) =
            find_file_with_pattern(&extracted_folder, &data_type.data_file_pattern)
        {
            {
                let time = std::time::Instant::now();
                let data_file = data_file_path.split('/').last().unwrap();

                let message = format!("Reading {}...", style(data_file).cyan());
                print_step_message(&message, count, DATA_TYPES.len(), READ);

                let _ = (data_type.read_function)(&data_file_path);

                let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
                let message = format!("{} loaded in {}s", style(data_file).cyan(), rounded_secs);
                print_step_message(&message, count, DATA_TYPES.len(), SUCCESS);
            }
        } else {
            let message = format!("File for {} not found", data_type.name);
            print_step_message(&message, count, DATA_TYPES.len(), ERROR);
        }
    }

    Ok(())
}

/**
 * Show the statistics.
 */
fn show_stats() -> Result<(), Box<dyn Error>> {
    let machines = MACHINES.lock().unwrap();
    let machines = machines.values().collect::<Vec<&Machine>>();

    let total_machines = machines.len();
    let total_clones = machines.iter().filter(|m| m.clone_of.is_some()).count();
    let total_originals = total_machines - total_clones;
    let unique_manufacturers: HashSet<_> = machines.iter().map(|m| &m.manufacturer).collect();
    let total_manufacturers = unique_manufacturers.len();
    let unique_series: HashSet<_> = machines.iter().map(|m| &m.series).collect();
    let total_series = unique_series.len();
    let unique_genres = machines.iter().map(|m| &m.genre).collect::<HashSet<_>>();
    let total_genres = unique_genres.len();
    let total_machines_with_history = machines
        .iter()
        .filter(|m| m.history_sections.len() > 0)
        .count();

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("MAME information statistics").style_spec("H2cFg")
    ]));

    table.add_row(row![b -> "Information", "Amount"]);
    table.add_row(row!["Machines", r -> total_machines.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Originals", r -> total_originals.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Clones", r -> total_clones.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Manufacturers", r -> total_manufacturers.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Series", r -> total_series.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Genres", r -> total_genres.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Machines with history", r -> total_machines_with_history.to_formatted_string(&Locale::en)]);

    table.printstd();

    Ok(())
}

/**
 * Create the SQLite database.
 */
fn create_database() -> Result<(), Box<dyn Error>> {
    let data_base_path = format!("{}{}", PATHS.export_path, "machines.db");

    let time = std::time::Instant::now();

    let message = format!("Creating {} database", style("machines.db").cyan());
    println_step_message(&message, 1, 1, WRITE);

    db_writer::write_machines(&data_base_path)?;

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Database created in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}

/**
 * Create the JSON file.
 */
fn create_json() -> Result<(), Box<dyn Error>> {
    let json_base_path = format!("{}{}", PATHS.export_path, "machines.json");

    let time = std::time::Instant::now();

    let message = format!("Creating {} JSON file", style("machines.json").cyan());
    println_step_message(&message, 1, 1, WRITE);

    json_writer::write_machines(&json_base_path)?;

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("JSON file created in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}
