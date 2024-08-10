use crate::core::data_types::DATA_TYPES;
use crate::helpers::{
    data_source_helper::get_data_source,
    file_download_helper::download_file,
    file_extractor_helper::extract_file,
    fs_helper::{find_file_with_pattern, get_file_name, PATHS},
    ui_helper::{icons::*, print_step_message, println_step_message},
};
use dialoguer::{console::style, theme::ColorfulTheme, Select};
use std::{error::Error, path::Path};

/**
 * Show the filter submenu.
 */
pub fn show_import_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &["Download files", "Extract files", "Read files", "< Back"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => download_files()?,
            1 => extract_files()?,
            2 => read_files()?,
            3 => {
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
        let message = format!("Getting URL for {}...", data_type.name);
        println_step_message(&message, count, DATA_TYPES.len(), DOWNLOAD);

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

        // Check if the zip file is present
        if let Some(zip_file_path) =
            find_file_with_pattern(&PATHS.download_path, &data_type.zip_file_pattern)
        {
            let zip_file = zip_file_path.split('/').last().unwrap();

            let message = format!(
                "Extracting {} to {}",
                style(zip_file).cyan(),
                extracted_folder
            );
            print_step_message(&message, count, DATA_TYPES.len(), FOLDER);

            extract_file(&zip_file_path, &extracted_folder)?;

            let message = format!("{} extracted successfully", style(zip_file).cyan());
            print_step_message(&message, count, DATA_TYPES.len(), SUCCESS);
        } else {
            let message = format!(
                "File for {} not found, please download first",
                data_type.name
            );
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
