
mod core;
mod helpers;

use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

use core::models::Machine;
use helpers::data_source_helper::get_data_source;
use helpers::file_download_helper::download_file;
use helpers::file_extractor_helper::extract_file;
use helpers::ui_helper::{
    icons::*, print_step_message, println_step_message, show_splash_screen, show_title,
};
use lazy_static::lazy_static;
use serde_json::to_string_pretty;
use core::data_types::DATA_TYPES;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

struct Paths {
    data_path: &'static str,
    download_path: &'static str,
    extracted_path: &'static str,
}

const PATHS: Paths = Paths {
    data_path: "data/",
    download_path: "data/downloads/",
    extracted_path: "data/extracted/",
};

lazy_static! {
    static ref URL_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref MACHINES: Arc<Mutex<HashMap<String, Machine>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn main() -> Result<(), Box<dyn Error>> {
    check_folder_structure()?;
    show_menu()?;
    Ok(())
}

/**
 * Get the URL from the URL map.
 */
fn get_url_from_map(name: &str) -> Option<String> {
    let map = URL_MAP.lock().unwrap();
    map.get(name).cloned()
}

/**
 * Set the URL in the URL map.
 */
fn set_url_in_map(name: &str, url: &str) {
    let mut map = URL_MAP.lock().unwrap();
    map.insert(name.to_string(), url.to_string());
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
 * Check if the required folder structure exists and create it if it doesn't.
 */
fn check_folder_structure() -> Result<(), Box<dyn Error>> {
    let paths = [PATHS.data_path, PATHS.download_path, PATHS.extracted_path];

    for path in paths.iter() {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
    }

    for data_type in DATA_TYPES.iter() {
        let subfolder = format!("{}/{}", PATHS.extracted_path, data_type.name.to_lowercase());
        if !Path::new(&subfolder).exists() {
            fs::create_dir_all(&subfolder)?;
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

        let message = format!("Getting URL for {}...", data_type.name);
        println_step_message(&message, count, DATA_TYPES.len(), DOWNLOAD);

        if let Ok(source_url) = get_data_source(data_type.source, data_type.source_match) {
            set_url_in_map(data_type.name, &source_url);

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
            find_file_with_pattern(&extracted_folder, &data_type.file_name_pattern)
        {
            let data_file = existing_file_path.split('/').last().unwrap();

            let message = format!("{} already exists (skipped)", style(data_file).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), INFO);

            continue;
        }

        // Get the file name from the URL but only if the URL is found
        let file_name = match get_url_from_map(data_type.name) {
            Some(url) => {
                // Get the file name from the URL
                let file_name = get_file_name(&url);
                file_name
            }
            None => {
                let message = format!("URL for {} not found", data_type.name);
                print_step_message(&message, count, DATA_TYPES.len(), ERROR);

                continue;
            }
        };

        // Get the file path
        let file_path = format!("{}{}", PATHS.download_path, file_name);

        // Check if the file exists
        if Path::new(&file_path).exists() {
            let message = format!("Extracting {} to {}", file_path, extracted_folder);
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
            find_file_with_pattern(&extracted_folder, &data_type.file_name_pattern)
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

/**
 * Find a file with the given pattern in the given folder.
 */
fn find_file_with_pattern(folder: &str, pattern: &regex::Regex) -> Option<String> {
    for entry in walkdir::WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if pattern.is_match(file_name) {
                    // return Some(path.to_path_buf());
                    return Some(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    None
}

/**
 * Get the file name from the given URL.
 */
fn get_file_name(url: &str) -> String {
    let last_param = url.split('/').last().unwrap_or("");
    let file_name = last_param.split('=').last().unwrap_or("");
    file_name.to_string()
}
