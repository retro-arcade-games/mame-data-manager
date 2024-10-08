mod helpers;
mod modules;
use dialoguer::{theme::ColorfulTheme, Select};
use helpers::ui_helper::{show_splash_screen, show_title};
use lazy_static::lazy_static;
use mame_parser::models::Machine;
use modules::{data_export, data_filtering, data_import, data_stats};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref MACHINES: Arc<Mutex<HashMap<String, Machine>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

fn main() -> Result<(), Box<dyn Error>> {
    show_main_menu()?;
    Ok(())
}

/// Show the main menu.
fn show_main_menu() -> Result<(), Box<dyn Error>> {
    show_splash_screen();
    show_title();

    loop {
        let selections = &[
            "Input data >",
            "Filter data >",
            "View statistics >",
            "Export data >",
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
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
