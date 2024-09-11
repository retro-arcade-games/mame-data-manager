use crate::helpers::ui_helper::{icons::*, print_message, println_message, show_section};
use crate::MACHINES;
use dialoguer::{console::style, theme::ColorfulTheme, Select};
use indicatif::{ProgressBar, ProgressStyle};
use mame_parser::file_handling::write_files;
use mame_parser::models::ExportFileType;
use mame_parser::progress::{CallbackType, ProgressCallback, ProgressInfo};
use std::error::Error;
use std::path::Path;

/**
 * Show the export submenu.
 */
pub fn show_export_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "Export to SQLite",
            "Export to JSON",
            "Export to CSV",
            "Back",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => export_sqlite()?,
            1 => export_json()?,
            2 => export_csv()?,
            3 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Create the SQLite database.
 */
fn export_sqlite() -> Result<(), Box<dyn Error>> {
    show_section("Export to SQLite");

    let workspace_path = Path::new("data");

    let machines = MACHINES.lock().unwrap();

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .progress_chars("#>-"),
    );

    let progress_callback: ProgressCallback = Box::new(move |progress_info: ProgressInfo| {
        // Update the progress bar
        match progress_info.callback_type {
            CallbackType::Progress => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
            }
            CallbackType::Info => {
                progress_bar.set_message(progress_info.message);
            }
            CallbackType::Finish => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
                progress_bar.finish_with_message(progress_info.message);
            }
            CallbackType::Error => {
                progress_bar.finish_with_message(progress_info.message);
            }
        }
    });

    let time = std::time::Instant::now();

    let message = format!("Creating {} database", style("machines.db").cyan());
    println_message(&message, WRITE);

    let result = write_files(
        ExportFileType::Sqlite,
        workspace_path,
        &machines,
        progress_callback,
    );

    if result.is_err() {
        let message = format!("Error: {}", result.err().unwrap());
        print_message(&message, ERROR);
        println!();
        return Ok(());
    }

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Database created in {}s", rounded_secs);
    print_message(&message, SUCCESS);

    println!();

    Ok(())
}

/**
 * Create the JSON file.
 */
fn export_json() -> Result<(), Box<dyn Error>> {
    show_section("Export to JSON");

    let workspace_path = Path::new("data");

    let machines = MACHINES.lock().unwrap();

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .progress_chars("#>-"),
    );

    let progress_callback: ProgressCallback = Box::new(move |progress_info: ProgressInfo| {
        // Update the progress bar
        match progress_info.callback_type {
            CallbackType::Progress => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
            }
            CallbackType::Info => {
                progress_bar.set_message(progress_info.message);
            }
            CallbackType::Finish => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
                progress_bar.finish_with_message(progress_info.message);
            }
            CallbackType::Error => {
                progress_bar.finish_with_message(progress_info.message);
            }
        }
    });

    let time = std::time::Instant::now();

    let message = format!("Creating JSON files");
    println_message(&message, WRITE);

    let result = write_files(
        ExportFileType::Json,
        workspace_path,
        &machines,
        progress_callback,
    );

    if result.is_err() {
        let message = format!("Error: {}", result.err().unwrap());
        print_message(&message, ERROR);
        println!();
        return Ok(());
    }

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("JSON files created in {}s", rounded_secs);
    print_message(&message, SUCCESS);

    println!();

    Ok(())
}

/**
 * Create the CSV file.
 */
fn export_csv() -> Result<(), Box<dyn Error>> {
    show_section("Export to CSV");

    let workspace_path = Path::new("data");

    let machines = MACHINES.lock().unwrap();

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:20.cyan/blue}] {pos}/{len} ({eta}) {msg}")
            .progress_chars("#>-"),
    );

    let progress_callback: ProgressCallback = Box::new(move |progress_info: ProgressInfo| {
        // Update the progress bar
        match progress_info.callback_type {
            CallbackType::Progress => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
            }
            CallbackType::Info => {
                progress_bar.set_message(progress_info.message);
            }
            CallbackType::Finish => {
                progress_bar.set_length(progress_info.total);
                progress_bar.set_position(progress_info.progress);
                progress_bar.finish_with_message(progress_info.message);
            }
            CallbackType::Error => {
                progress_bar.finish_with_message(progress_info.message);
            }
        }
    });

    let time = std::time::Instant::now();

    let message = format!("Creating CSV files");
    println_message(&message, WRITE);

    let result = write_files(
        ExportFileType::Csv,
        workspace_path,
        &machines,
        progress_callback,
    );

    if result.is_err() {
        let message = format!("Error: {}", result.err().unwrap());
        print_message(&message, ERROR);
        println!();
        return Ok(());
    }

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("CSV files created in {}s", rounded_secs);
    print_message(&message, SUCCESS);

    println!();

    Ok(())
}
