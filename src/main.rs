mod core;
mod helpers;
mod modules;
use core::data::get_list;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::fs_helper::check_folder_structure;
use helpers::ui_helper::{show_splash_screen, show_title};
use modules::{data_export, data_filtering, data_import, data_stats};
use std::error::Error;

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
    show_title();

    loop {
        let selections = &[
            "Input data >",
            "Filter data >",
            "View statistics >",
            "Export data >",
            "View lists",
            "Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => data_import::show_import_submenu()?,
            1 => data_filtering::show_filtering_submenu()?,
            2 => data_stats::show_stats_submenu()?,
            3 => data_export::show_export_submenu()?,
            4 => {
                // Print MACHINES len
                let machines = core::data::MACHINES.lock().unwrap();
                println!("Machines: {}", machines.len());
                // Print series len
                println!("Series: {}", get_list(&core::data::SERIES).len());
                // Print top 10 series
                let top_series = core::data::get_top(&core::data::SERIES, 10);
                for (name, count) in top_series {
                    println!("{}: {}", name, count);
                }
                // Print manufacturers len
                println!("Manufacturers: {}", get_list(&core::data::MANUFACTURERS).len());
                // Print top 10 manufacturers
                let top_manufacturers = core::data::get_top(&core::data::MANUFACTURERS, 10);
                for (name, count) in top_manufacturers {
                    println!("{}: {}", name, count);
                }
                let players = core::data::PLAYERS.lock().unwrap();
                println!("Players: {}", players.len());
                let languages = core::data::LANGUAGES.lock().unwrap();
                println!("Languages: {}", languages.len());
                let categories = core::data::CATEGORIES.lock().unwrap();
                println!("Categories: {}", categories.len());
                let subcategories = core::data::SUBCATEGORIES.lock().unwrap();
                println!("Subcategories: {}", subcategories.len());
            }
            5 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
