use crate::core::writers::{db_writer, json_writer};
use crate::helpers::fs_helper::PATHS;
use crate::helpers::ui_helper::{icons::*, print_step_message, println_step_message, show_section};
use dialoguer::{console::style, theme::ColorfulTheme, Select};
use std::error::Error;

/**
 * Show the export submenu.
 */
pub fn show_export_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "Export to SQLite",
            "Export to JSON",
            "Export to ODS",
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
            2 => export_ods()?,
            3 => export_csv()?,
            4 => {
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

    let data_base_path = format!("{}{}", PATHS.export_path, "machines.db");

    let time = std::time::Instant::now();

    let message = format!("Creating {} database", style("machines.db").cyan());
    println_step_message(&message, 1, 1, WRITE);

    db_writer::write_machines(&data_base_path)?;

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Database created in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    println!();

    Ok(())
}

/**
 * Create the JSON file.
 */
fn export_json() -> Result<(), Box<dyn Error>> {
    show_section("Export to JSON");

    let json_base_path = format!("{}{}", PATHS.export_path, "machines.json");

    let time = std::time::Instant::now();

    let message = format!("Creating {} JSON file", style("machines.json").cyan());
    println_step_message(&message, 1, 1, WRITE);

    json_writer::write_machines(&json_base_path)?;

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("JSON file created in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    println!();

    Ok(())
}

/**
 * Create the ODS file.
 */
fn export_ods() -> Result<(), Box<dyn Error>> {
    show_section("Export to ODS");

    // let ods_base_path = format!("{}{}", PATHS.export_path, "machines.ods");

    // let time = std::time::Instant::now();

    let message = format!("Creating {} ODS file", style("machines.ods").cyan());
    println_step_message(&message, 1, 1, WRITE);

    todo!("Implement the ODS writer");

    // let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    // let message = format!("JSON file created in {}s", rounded_secs);
    // print_step_message(&message, 1, 1, SUCCESS);

    // println!();

    // Ok(())
}

/**
 * Create the CSV file.
 */
fn export_csv() -> Result<(), Box<dyn Error>> {
    show_section("Export to CSV");

    // let csv_base_path = format!("{}{}", PATHS.export_path, "machines.csv");

    // let time = std::time::Instant::now();

    let message = format!("Creating {} CSV file", style("machines.csv").cyan());
    println_step_message(&message, 1, 1, WRITE);

    todo!("Implement the CSV writer");

    // let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    // let message = format!("CSV file created in {}s", rounded_secs);
    // print_step_message(&message, 1, 1, SUCCESS);

    // println!();

    // Ok(())
}
