mod core;
mod helpers;
mod modules;

use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::fs_helper::{check_folder_structure, PATHS};
use modules::{data_filtering, input_data_management};
use num_format::{Locale, ToFormattedString};

use core::data::MACHINES;
use core::models::Machine;
use core::writers::{db_writer, json_writer};
use helpers::ui_helper::{
    icons::*, print_step_message, println_step_message, show_splash_screen, show_title,
};
use std::collections::HashSet;
use std::error::Error;

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn main() -> Result<(), Box<dyn Error>> {
    check_folder_structure()?;
    show_main_menu()?;
    Ok(())
}

/**
 * Show the main menu.
 */
fn show_main_menu() -> Result<(), Box<dyn Error>> {
    show_splash_screen();

    loop {
        show_title();

        let selections = &[
            "Manage input data >",
            "Filter data >",
            "View statistics >",
            "Export information >",
            "Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => input_data_management::show_input_data_submenu()?,
            1 => data_filtering::show_filtering_submenu()?,
            2 => show_stats()?,
            3 => show_export_submenu()?,
            4 => {
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
