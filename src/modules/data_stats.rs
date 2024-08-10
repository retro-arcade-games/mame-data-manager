use crate::core::{data::MACHINES, models::Machine};
use dialoguer::{theme::ColorfulTheme, Select};
use num_format::{Locale, ToFormattedString};
use prettytable::{row, Cell, Row, Table};
use std::collections::HashSet;
use std::error::Error;

/**
 * Show the filter submenu.
 */
pub fn show_stats_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &["View all", "< Back"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => show_stats()?,
            1 => {
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
    let unique_categories = machines.iter().map(|m| &m.category).collect::<HashSet<_>>();
    let total_categories = unique_categories.len();
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
    table.add_row(row!["Categories", r -> total_categories.to_formatted_string(&Locale::en)]);
    table.add_row(row!["Machines with history", r -> total_machines_with_history.to_formatted_string(&Locale::en)]);

    table.printstd();
    println!();
    Ok(())
}
