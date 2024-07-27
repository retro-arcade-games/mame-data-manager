use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("/ Mame Data Manager");
        println!("===================");

        let selections = &["Download files", "Read files", "Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => download_files()?,
            1 => read_files()?,
            2 => {
                println!("Exiting...");
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn download_files() -> Result<(), Box<dyn Error>> {
    // TODO: Implement the functionality for downloading files
    println!("Download files option selected.");
    Ok(())
}

fn read_files() -> Result<(), Box<dyn Error>> {
    // TODO: Implement the functionality for reading files
    println!("Read files option selected.");
    Ok(())
}
