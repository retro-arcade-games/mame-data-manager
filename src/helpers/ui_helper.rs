use console::{style, Emoji, Term};
use lazy_static::lazy_static;
use std::io;

pub mod icons {
    use console::Emoji;

    pub static DOWNLOAD: Emoji<'_, '_> = Emoji("🌐 ", "");
    pub static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "");
    pub static ERROR: Emoji<'_, '_> = Emoji("🚨 ", "");
    pub static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "");
    pub static LOUPE: Emoji<'_, '_> = Emoji("🔍 ", "");
    pub static FOLDER: Emoji<'_, '_> = Emoji("🗂 ", "");
    pub static READ: Emoji<'_, '_> = Emoji("🧾 ", "");
}

lazy_static! {
    static ref TERM: Term = Term::stdout();
}

/**
 * Clear the console screen.
 */
pub fn clear_console() {
    TERM.clear_screen().unwrap();
}

// function that receives a message, current step and total steps and an icon and prints a message to the console
pub fn print_step_message(message: &str, step: usize, total_steps: usize, icon: Emoji) {

    clean_last_line().unwrap_or_default();

    println_step_message(message, step, total_steps, icon)
}

pub fn println_step_message(message: &str, step: usize, total_steps: usize, icon: Emoji) {

    let step = format!("[{}/{}]", step, total_steps);

    println!(
        "{} {} {}",
        style(step).bold().dim(),
        icon,
        message,
    );
}

pub fn show_splash_screen() {
    clear_console();
    println!(" █    ███         ███    █");
    println!("███    ███       ███    ███");
    println!("███  █████████████████  ███");
    println!("████████ █████████ ████████");
    println!("███████████████████████████");
    println!("███████████████████████████");
    println!("  ███████████████████████  ");
    println!("     ███           ███     ");
    println!("   ████             ████   ");
}

pub fn show_title() {
    TERM.set_title("Mame Data Manager");
    println!();
    println!("===========================");
    println!("/    Mame Data Manager    /");
    println!("===========================");
    println!();
}

fn clean_last_line() -> Result<(), io::Error> {
    TERM.move_cursor_up(1)?;
    TERM.clear_line()?;
    Ok(())
}