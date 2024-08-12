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
                println!("---------------------------------");
                // Print series len
                println!("Series: {}", get_list(&core::data::SERIES).len());
                // Print top 10 series
                let top_series = core::data::get_top(&core::data::SERIES, 10);
                for (name, count) in top_series {
                    println!("{}: {}", name, count);
                }
                println!("---------------------------------");
                // Print manufacturers len
                println!(
                    "Manufacturers: {}",
                    get_list(&core::data::MANUFACTURERS).len()
                );
                // Print top 10 manufacturers
                let top_manufacturers = core::data::get_top(&core::data::MANUFACTURERS, 10);
                for (name, count) in top_manufacturers {
                    println!("{}: {}", name, count);
                }
                println!("---------------------------------");
                // Print players len
                println!("Players: {}", get_list(&core::data::PLAYERS).len());
                // Print top 10 players
                let top_players = core::data::get_top(&core::data::PLAYERS, 10);
                for (name, count) in top_players {
                    println!("{}: {}", name, count);
                }
                println!("---------------------------------");
                // Print languages len
                println!("Languages: {}", get_list(&core::data::LANGUAGES).len());
                // Print top 10 languages
                let top_languages = core::data::get_top(&core::data::LANGUAGES, 10);
                for (name, count) in top_languages {
                    println!("{}: {}", name, count);
                }
            }
            5 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
