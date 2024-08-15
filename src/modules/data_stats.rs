use crate::core::data::{
    get_top, CATEGORIES, LANGUAGES, MANUFACTURERS, PLAYERS, SERIES, SUBCATEGORIES,
};
use crate::core::{data::MACHINES, models::Machine};
use crate::helpers::ui_helper::icons::ERROR;
use crate::helpers::ui_helper::{println_message, show_section};
use dialoguer::{theme::ColorfulTheme, Select};
use num_format::{Locale, ToFormattedString};
use prettytable::{row, Cell, Row, Table};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

/**
 * Show the filter submenu.
 */
pub fn show_stats_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "General stats",
            "Top 10 categories",
            "Top 10 subcategories",
            "Top 10 manufacturers",
            "Top 10 series",
            "Top 10 languages",
            "Top 10 players information",
            "< Back",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => show_stats()?,
            1 => show_top_by_collection(
                "Top categories".to_string(),
                "Category".to_string(),
                &CATEGORIES,
            )?,
            2 => show_top_by_collection(
                "Top subcategories".to_string(),
                "Category - Subcategory".to_string(),
                &SUBCATEGORIES,
            )?,
            3 => show_top_by_collection(
                "Top manufacturers".to_string(),
                "Manufacturer".to_string(),
                &MANUFACTURERS,
            )?,
            4 => show_top_by_collection("Top series".to_string(), "Series".to_string(), &SERIES)?,
            5 => show_top_by_collection(
                "Top languages".to_string(),
                "Language".to_string(),
                &LANGUAGES,
            )?,
            6 => show_top_by_collection(
                "Top players information".to_string(),
                "Player".to_string(),
                &PLAYERS,
            )?,
            7 => {
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
    show_section("General stats");

    let machines = MACHINES.lock().unwrap();

    if machines.is_empty() {
        let message = format!(
            "Error: {}",
            "No machines data loaded, please read the data first."
        );
        println_message(&message, ERROR);
        println!();
        return Ok(());
    }

    let machines = machines.values().collect::<Vec<&Machine>>();

    let total_machines = machines.len();
    let total_clones = machines.iter().filter(|m| m.clone_of.is_some()).count();
    let total_originals = total_machines - total_clones;

    let total_manufacturers = MANUFACTURERS.lock().unwrap().len();
    let total_categories = CATEGORIES.lock().unwrap().len();
    let total_subcategories = SUBCATEGORIES.lock().unwrap().len();
    let total_series = SERIES.lock().unwrap().len();
    let total_languages = LANGUAGES.lock().unwrap().len();
    let total_players = PLAYERS.lock().unwrap().len();

    let total_machines_with_history = machines
        .iter()
        .filter(|m| m.history_sections.len() > 0)
        .count();

    let total_machines_with_resources = machines.iter().filter(|m| m.resources.len() > 0).count();

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("MAME information statistics").style_spec("H2cFg")
    ]));

    table.add_row(row![b -> "Information", "Amount"]);
    table.add_row(row!["Machines", r -> total_machines.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Originals", r -> total_originals.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Clones", r -> total_clones.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Manufacturers", r -> total_manufacturers.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Categories", r -> total_categories.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Subcategories", r -> total_subcategories.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Series", r -> total_series.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Languages", r -> total_languages.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Players information", r -> total_players.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Machines with history", r -> total_machines_with_history.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Machines with resources", r -> total_machines_with_resources.to_formatted_string(&Locale::en)]);

    table.printstd();

    println!();

    Ok(())
}

/**
 * Show the top by collection.
 */
fn show_top_by_collection(
    title: String,
    column: String,
    map: &Arc<Mutex<HashMap<String, usize>>>,
) -> Result<(), Box<dyn Error>> {
    show_section(&title);

    let machines = MACHINES.lock().unwrap();

    if machines.is_empty() {
        let message = format!(
            "Error: {}",
            "No machines data loaded, please read the data first."
        );
        println_message(&message, ERROR);
        println!();
        return Ok(());
    }

    let top = get_top(map, 10);

    let mut table = Table::new();
    table.set_titles(Row::new(vec![Cell::new(&title).style_spec("H3cFg")]));

    table.add_row(row![b -> "#", column, "Machines"]);

    let mut counter = 0;
    for (item, count) in top {
        counter += 1;
        table.add_row(row![counter, item, count.to_formatted_string(&Locale::en)]);
    }

    table.printstd();

    println!();

    Ok(())
}
