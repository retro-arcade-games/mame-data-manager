use console::{style, Emoji, Term};
use lazy_static::lazy_static;
use std::io;

pub mod icons {
    use console::Emoji;
    pub static ERROR: Emoji<'_, '_> = Emoji("🚨 ", "");
    pub static SUCCESS: Emoji<'_, '_> = Emoji("✅ ", "");
    pub static WRITE: Emoji<'_, '_> = Emoji("📝 ", "");
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

/**
 * Print a message with an icon.
 */
pub fn println_message(message: &str, icon: Emoji) {
    println!("{} {}", icon, message,);
}

/**
 * Print a message with an icon on the same line.
 */
pub fn print_message(message: &str, icon: Emoji) {
    clean_last_line().unwrap_or_default();
    println_message(message, icon)
}

/**
 * Show the splash screen.
 */
pub fn show_splash_screen() {
    clear_console();
    println!("           █████████           ");
    println!("          ░█   █████▓          ");
    println!("        ██     ████████        ");
    println!("        ███████████████        ");
    println!("           █████████           ");
    println!("             █████             ");
    println!("     █████  ▒█████░▒▓▓███      ");
    println!(" ████        █████        ███▒ ");
    println!("██        ████████████       ██");
    println!("█████                     █████");
    println!("     ██████████████████████░   ");
}

/**
 * Show the title.
 */
pub fn show_title() {
    TERM.set_title("Mame Data Manager");
    println!();
    println!("===============================");
    println!("/      Mame Data Manager      /");
    println!("===============================");
    println!();
}

/**
 * Show a section.
 */
pub fn show_section(section: &str) {
    println!("-- {} --", style(section).bold());
}

/**
 * Clean the last line.
 */
fn clean_last_line() -> Result<(), io::Error> {
    TERM.move_cursor_up(1)?;
    TERM.clear_line()?;
    Ok(())
}
