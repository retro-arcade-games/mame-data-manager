use std::error::Error;

use crate::core::writers::db_writer;
use crate::helpers::ui_helper::{icons::*, print_step_message, println_step_message};
use crate::{core::writers::json_writer, helpers::fs_helper::PATHS};
use dialoguer::{console::style, theme::ColorfulTheme, Select};

/**
 * Show the export submenu.
 */
pub fn show_export_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &["Export to SQLite", "Export to JSON", "Back"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => export_sqlite()?,
            1 => export_json()?,
            2 => {
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
fn export_json() -> Result<(), Box<dyn Error>> {
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
