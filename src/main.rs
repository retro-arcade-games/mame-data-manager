mod data_types;
mod helpers;
mod readers;
mod models;

use lazy_static::lazy_static;
use models::Machine;
use data_types::DATA_TYPES;
use helpers::file_download_helper::download_file;
use helpers::file_extractor_helper::extract_file;
use helpers::data_source_helper::get_data_source;
use dialoguer::{theme::ColorfulTheme, Select}; 
use std::{fs, io};
use std::error::Error;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use serde_json::to_string_pretty;
use console::{style, Emoji, Term};

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

static DOWNLOAD: Emoji<'_, '_> = Emoji("🌐 ", "");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "");
static ERROR: Emoji<'_, '_> = Emoji("🚨 ", ":-)");
static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", ":-)");

lazy_static! {
    static ref URL_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref MACHINES: Mutex<HashMap<String, Machine>> = Mutex::new(HashMap::new());
    static ref TERM: Term = Term::stdout();
}

fn main() -> Result<(), Box<dyn Error>> {
    clear_console();
    check_folder_structure()?;
    show_menu()?;
    Ok(())
}

/**
 * Clear the console screen.
 */
fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
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
    loop {
        println!("/ Mame Data Manager");
        println!("===================");

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

fn clean_last_line() -> Result<(), io::Error> {
    TERM.move_cursor_up(1)?;
    TERM.clear_line()?;
    Ok(())
}

/**
 * Download the files from the data sources.
 */
fn download_files() -> Result<(), Box<dyn Error>> {
    let mut count = 0;

    for data_type in DATA_TYPES.iter() {
        
        count += 1;
        let step = format!("[{}/{}]", count, DATA_TYPES.len());

        println!(
            "{} {} Getting URL for {}...",
            style(step.clone()).bold().dim(),
            DOWNLOAD,
            data_type.name
        );

        if let Ok(source_url) = get_data_source(data_type.source, data_type.source_match) {
            
            set_url_in_map(data_type.name, &source_url);

            let file_name = get_file_name(&source_url);
            let file_path = format!("{}{}", PATHS.download_path, file_name);

            clean_last_line()?;

            if !Path::new(&file_path).exists() {
                
                println!(
                    "{} {} Downloading {}...",
                    style(step.clone()).bold().dim(),
                    DOWNLOAD,
                    source_url
                );
                
                download_file(&source_url, &file_path)?;
                
                clean_last_line()?;
                
                println!(
                    "{} {} {} downloaded successfully",
                    style(step.clone()).bold().dim(),
                    SUCCESS,
                    style(file_name).cyan()
                );

            } else {

                println!(
                    "{} {} {} already exists (skipped)",
                    style(step).bold().dim(),
                    INFO,
                    style(file_name).cyan()
                );
            }
        } else {

            clean_last_line()?;

            println!(
                "{} {} Failed getting matching source for {}",
                style(step).bold().dim(),
                ERROR,
                data_type.name
            );
        }
    }

    Ok(())
}

/**
 * Extract the downloaded files.
 */
fn extract_files() -> Result<(), Box<dyn Error>> {
    for data_type in DATA_TYPES.iter() {

        let extracted_folder = format!("{}{}", PATHS.extracted_path, data_type.name.to_lowercase());

        // Check if the file already exists in the extracted folder
        if let Some(existing_file_path) = find_file_with_pattern(&extracted_folder, &data_type.file_name_pattern) {
            println!("File {} already exists, skipping extraction.", existing_file_path);
            continue;
        }

        // Get the file name from the URL but only if the URL is found
        let file_name = match get_url_from_map(data_type.name) {
            Some(url) => {
                // Get the file name from the URL
                let file_name = get_file_name(&url);
                file_name
            },
            None => {
                println!("URL for {} not found, skipping.", data_type.name);
                continue;
            }
        };

        // Get the file path
        let file_path = format!("{}{}", PATHS.download_path, file_name);

        // Check if the file exists
        if Path::new(&file_path).exists() {
            println!("Extracting file {} to {}", file_path, extracted_folder);
            extract_file(&file_path, &extracted_folder)?;

        } else {
            println!("File for {} not found, skipping.", file_name);
        }
        
    }
    Ok(())
}

/**
 * Read the extracted files.
 */
fn read_files() -> Result<(), Box<dyn Error>> {
    for data_type in DATA_TYPES.iter() {
        let extracted_folder = format!("{}{}", PATHS.extracted_path, data_type.name.to_lowercase());

        if let Some(data_file_path) = find_file_with_pattern(&extracted_folder, &data_type.file_name_pattern) {
            {
                let mut machines_guard = MACHINES.lock().unwrap();
                let _ = (data_type.read_function)(&data_file_path, &mut machines_guard);
            }
        } else {
            println!("No file found with the given pattern for data type: {}", data_type.name);
        }
    }
    let machines_guard = MACHINES.lock().unwrap();
    if let Some(machine) = machines_guard.get("mk") {
        let json_data = to_string_pretty(&machine.name).expect("Failed to serialize machine to JSON");
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
    for entry in walkdir::WalkDir::new(folder).into_iter().filter_map(|e| e.ok()) {
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