mod data_types;
mod helpers;

use data_types::{DATA_TYPES};
use helpers::file_download_helper::download_file;
use helpers::data_source_helper::get_data_source;
use dialoguer::{theme::ColorfulTheme, Select}; 
use std::fs;
use std::path::Path;
use std::error::Error;

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

fn main() -> Result<(), Box<dyn Error>> {

    check_folder_structure()?;

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

fn check_folder_structure() -> Result<(), Box<dyn Error>> {
    let paths = [PATHS.data_path, PATHS.download_path, PATHS.extracted_path];

    for path in paths.iter() {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}

fn download_files() -> Result<(), Box<dyn Error>> {
    for data_type in DATA_TYPES.iter() {
        
        if let Ok(source_url) = get_data_source(data_type.source, data_type.source_match) {
            let file_name = get_file_name(&source_url);
            let file_path = format!("{}{}", PATHS.download_path, file_name);

            if !Path::new(&file_path).exists() {
                println!("Downloading {} from {}", data_type.name, source_url);
                download_file(&source_url, &file_path)?;
            } else {
                println!("File for {} already exists, skipping.", data_type.name);
            }
        } else {
            eprintln!("Error: failed to find a matching source for {}", data_type.name);
        }
    }

    Ok(())
}

fn extract_files() -> Result<(), Box<dyn Error>> {
    // TODO: Implement the functionality for extracting files
    println!("Extract files option selected.");
    Ok(())
}

fn read_files() -> Result<(), Box<dyn Error>> {
    // TODO: Implement the functionality for reading files
    println!("Read files option selected.");
    Ok(())
}

fn get_file_name(url: &str) -> String {
    let last_param = url.split('/').last().unwrap_or("");
    let file_name = last_param.split('=').last().unwrap_or("");
    file_name.to_string()
}