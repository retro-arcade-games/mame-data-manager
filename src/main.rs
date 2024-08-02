mod core;
mod helpers;

use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::fs_helper::{check_folder_structure, find_file_with_pattern, get_file_name, PATHS};

use core::data_types::{DataType, DATA_TYPES};
use core::models::Machine;
use helpers::data_source_helper::get_data_source;
use helpers::file_download_helper::download_file;
use helpers::file_extractor_helper::extract_file;
use helpers::ui_helper::{
    icons::*, print_step_message, println_step_message, show_splash_screen, show_title,
};
use lazy_static::lazy_static;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref MACHINES: Arc<Mutex<HashMap<String, Machine>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

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

        let selections = &["Download files", "Extract files", "Read files", "Exit"];
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
            3 => {
                println!("Exiting...");
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
            let message = format!("Downloading {}...", source_url);
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

        let extracted_folder = format!("{}{}", PATHS.extracted_path, data_type.name.to_lowercase());

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

        let extracted_folder = format!("{}{}", PATHS.extracted_path, data_type.name.to_lowercase());

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

                let mut machines_guard = MACHINES.lock().unwrap();
                let _ = (data_type.read_function)(&data_file_path, &mut machines_guard);

                let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
                let message = format!("{} loaded in {}s", style(data_file).cyan(), rounded_secs);
                print_step_message(&message, count, DATA_TYPES.len(), SUCCESS);
            }
        } else {
            let message = format!("File for {} not found", data_type.name);
            print_step_message(&message, count, DATA_TYPES.len(), ERROR);
        }
    }
    let machines_guard = MACHINES.lock().unwrap();
    if let Some(machine) = machines_guard.get("mk") {
        let json_data = to_string_pretty(&machine).expect("Failed to serialize machine to JSON");
        println!("Machine found: Name: mk, Data: {}", json_data);
    } else {
        println!("Machine with name 'mk' not found");
    }

    Ok(())
}
