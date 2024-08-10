use dialoguer::{theme::ColorfulTheme, Select};

use crate::core::filters::{
    filter_genres, filter_non_games, manufacturer_refactor, name_refactor, nplayers_refactor,
};

use crate::helpers::ui_helper::{icons::*, print_step_message, println_step_message};

use std::error::Error;

/**
 * Show the filter submenu.
 */
pub fn show_filtering_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &[
            "Refactor names",
            "Remove non game genres",
            "Remove non games",
            "Refactor manufacturers",
            "Refactor players",
            "< Back",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => refactor_names()?,
            1 => filter_genres()?,
            2 => filter_non_games()?,
            3 => refactor_manufacturers()?,
            4 => refactor_nplayers()?,
            5 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

/**
 * Refactor the names.
 */
fn refactor_names() -> Result<(), Box<dyn Error>> {
    let message = format!("Refactoring machine names");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = name_refactor::refactor_names();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Machine names refactored in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}

/**
 * Filter the categories.
 */
fn filter_genres() -> Result<(), Box<dyn Error>> {
    let message = format!("Removing non game machines by genre");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let removed_machines = filter_genres::filter_genres();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!(
        "{} machines removed in {}s",
        removed_machines?, rounded_secs
    );
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}

/**
 * Filter the non games.
 */
fn filter_non_games() -> Result<(), Box<dyn Error>> {
    let message = format!("Removing non game machines");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let removed_machines = filter_non_games::filter_non_games();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!(
        "{} machines removed in {}s",
        removed_machines?, rounded_secs
    );
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}

/**
 * Refactor the manufacturers.
 */
fn refactor_manufacturers() -> Result<(), Box<dyn Error>> {
    let message = format!("Refactoring manufacturers");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = manufacturer_refactor::refactor_manufacturers();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Manufacturers refactored in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}

/**
 * Refactor the number of players.
 */
fn refactor_nplayers() -> Result<(), Box<dyn Error>> {
    let message = format!("Refactoring number of players");
    println_step_message(&message, 1, 1, WRITE);

    let time = std::time::Instant::now();

    let _ = nplayers_refactor::refactor_nplayers();

    let rounded_secs = (time.elapsed().as_secs_f32() * 10.0).round() / 10.0;
    let message = format!("Number of players refactored in {}s", rounded_secs);
    print_step_message(&message, 1, 1, SUCCESS);

    Ok(())
}
