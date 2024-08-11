use crate::core::filters::machines_filtering::MachineFilter;
use crate::core::filters::{
    machine_names_normalization, machines_filtering, manufacturers_normalization,
    non_game_categories_removal, nplayers_normalization,
};
use crate::helpers::ui_helper::{icons::*, print_step_message, println_step_message, show_section};
use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;

/**
 * Show the filter submenu.
 */
pub fn show_filtering_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "Normalize machine names",
            "Normalize manufacturers",
            "Normalize number of players",
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
            0 => normalize_machine_names()?,
            1 => normalize_manufacturers()?,
            2 => normalize_nplayers()?,
            3 => remove_non_game_categories()?,
            4 => remove_non_games(MachineFilter::Device)?,
            5 => remove_non_games(MachineFilter::Bios)?,
            6 => remove_non_games(MachineFilter::Mechanical)?,
            7 => remove_non_games(MachineFilter::Modified)?,
            8 => remove_non_games(MachineFilter::All)?,
            9 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Normalize the machine names.
 */
fn normalize_machine_names() -> Result<(), Box<dyn Error>> {
    show_section("Normalize machine names");

    let message = format!("Normalizing machine names");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = machine_names_normalization::normalize_machine_names();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Machine names normalized in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);
    println!();
    Ok(())
}

/**
 * Normalize the manufacturers.
 */
fn normalize_manufacturers() -> Result<(), Box<dyn Error>> {
    show_section("Normalize manufacturers");

    let message = format!("Normalizing manufacturers");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = manufacturers_normalization::normalize_manufacturers();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Manufacturers normalized in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);
    println!();
    Ok(())
}

/**
 * Normalize the number of players.
 */
fn normalize_nplayers() -> Result<(), Box<dyn Error>> {
    show_section("Normalize number of players");

    let message = format!("Normalizing number of players");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = nplayers_normalization::normalize_nplayers();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Number of players normalized in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);
    println!();
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
