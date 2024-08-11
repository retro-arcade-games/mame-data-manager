use crate::core::filters::machines_filtering::MachineFilter;
use crate::core::filters::{machines_filtering, non_game_categories_removal};
use crate::helpers::ui_helper::{icons::*, print_step_message, println_step_message, show_section};
use dialoguer::{theme::ColorfulTheme, Select};
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
            "Remove ALL non game machines (apply all filters)",
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
            5 => remove_non_games(MachineFilter::All)?,
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
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let removed_machines = non_game_categories_removal::remove_non_game_categories();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!(
        "{} machines removed in {}s",
        removed_machines?, rounded_secs
    );
    print_step_message(&message, 1, 1, SUCCESS);
    println!();
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
        MachineFilter::All => "ALL",
    };

    let section_name = format!("Remove {} machines", filter_name);

    show_section(&section_name);

    let message = format!("Removing {} machines", filter_name);
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let removed_machines = machines_filtering::remove_machines_by_filter(remove_filter);

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!(
        "{} machines with filter {} removed in {}s",
        removed_machines?, filter_name, rounded_secs
    );
    print_step_message(&message, 1, 1, SUCCESS);
    println!();
    Ok(())
}
