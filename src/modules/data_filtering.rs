use crate::helpers::ui_helper::{icons::*, print_message, println_message, show_section};
use crate::MACHINES;
use dialoguer::{theme::ColorfulTheme, Select};
use mame_parser::file_handling::{remove_machines_by_category, remove_machines_by_filter};
use mame_parser::models::{Category, MachineFilter};
use std::error::Error;

/**
 * Show the filter submenu.
 */
pub fn show_filtering_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "Remove machines with non game categories",
            "Remove devices machines",
            "Remove bios machines",
            "Remove mechanical machines",
            "Remove modified machines",
            "Remove clones",
            "< Back",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => remove_non_game_categories()?,
            1 => remove_non_games(MachineFilter::Device)?,
            2 => remove_non_games(MachineFilter::Bios)?,
            3 => remove_non_games(MachineFilter::Mechanical)?,
            4 => remove_non_games(MachineFilter::Modified)?,
            5 => remove_non_games(MachineFilter::Clones)?,
            6 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Remove machines by non game categories.
 */
fn remove_non_game_categories() -> Result<(), Box<dyn Error>> {
    show_section("Remove machines by non game categories");

    let message = format!("Removing machines by non game categories");
    println_message(&message, WRITE);

    let time = std::time::Instant::now();

    let categories_to_remove = vec![
        Category::BoardGame,
        Category::Calculator,
        Category::CardGames,
        Category::Computer,
        Category::ComputerGraphicWorkstation,
        Category::DigitalCamera,
        Category::DigitalSimulator,
        Category::Electromechanical,
        Category::Game,
        Category::GameConsole,
        Category::GameConsoleComputer,
        Category::Handheld,
        Category::MedicalEquipment,
        Category::Misc,
        Category::MultiGame,
        Category::Multiplay,
        Category::Music,
        Category::Player,
        Category::Printer,
        Category::Radio,
        Category::Simulation,
        Category::SlotMachine,
        Category::System,
        Category::Tablet,
        Category::Tabletop,
        Category::Telephone,
        Category::Touchscreen,
        Category::TTLDriving,
        Category::TTLMaze,
        Category::TTLQuiz,
        Category::TTLShooter,
        Category::TTLSports,
        Category::TVBundle,
        Category::Utilities,
        Category::Watch,
    ];

    {
        let mut machines_guard = MACHINES.lock().unwrap();
        let filtered_machines = remove_machines_by_category(&machines_guard, &categories_to_remove);

        if filtered_machines.is_err() {
            let message = format!("Error: {}", filtered_machines.err().unwrap());
            print_message(&message, ERROR);
            println!();
            return Ok(());
        }

        let removed_machines = machines_guard.len() - filtered_machines.as_ref().unwrap().len();
        let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
        let message = format!("{} machines removed in {}s", removed_machines, rounded_secs);
        *machines_guard = filtered_machines?;
        print_message(&message, SUCCESS);
        println!();
    }

    Ok(())
}

/**
 * Remove machines by filter.
 */
fn remove_non_games(remove_filter: MachineFilter) -> Result<(), Box<dyn Error>> {
    let filter_name = match remove_filter {
        MachineFilter::Device => "Device",
        MachineFilter::Bios => "BIOS",
        MachineFilter::Mechanical => "Mechanical",
        MachineFilter::Modified => "Modified",
        MachineFilter::Clones => "Clones",
    };

    let section_name = format!("Remove {} machines", filter_name);

    show_section(&section_name);

    let message = format!("Removing {} machines", filter_name);
    println_message(&message, WRITE);

    let time = std::time::Instant::now();
    {
        let mut machines_guard = MACHINES.lock().unwrap();
        let filters_to_remove = vec![remove_filter];
        let filtered_machines = remove_machines_by_filter(&machines_guard, &filters_to_remove);

        if filtered_machines.is_err() {
            let message = format!("Error: {}", filtered_machines.err().unwrap());
            print_message(&message, ERROR);
            println!();
            return Ok(());
        }

        let removed_machines = machines_guard.len() - filtered_machines.as_ref().unwrap().len();
        let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
        let message = format!("{} machines removed in {}s", removed_machines, rounded_secs);
        *machines_guard = filtered_machines?;
        print_message(&message, SUCCESS);
        println!();
    }

    Ok(())
}
