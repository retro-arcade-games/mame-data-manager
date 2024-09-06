use crate::core::data::{recreate_lists, MACHINES};
use crate::helpers::ui_helper::show_section;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use mame_parser::models::MameDataType;
use mame_parser::progress::{CallbackType, ProgressInfo, SharedProgressCallback};
use std::sync::Arc;
use std::thread;
use std::{error::Error, path::Path};

/**
 * Show the filter submenu.
 */
pub fn show_import_submenu() -> Result<(), Box<dyn Error>> {
    loop {
        let selections = &["Download files", "Unpack files", "Read files", "< Back"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => download_all_files()?,
            1 => unpack_all_files()?,
            2 => read_all_files()?,
            3 => {
                break;
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}

fn download_all_files() -> Result<(), Box<dyn Error>> {
    show_section("Download Files");
    // Define the workspace path
    let workspace_path = Path::new("data");

    // Create a multi progress bar
    let multi_progress = MultiProgress::new();

    // Create progress bars for each data type
    let progress_bars = Arc::new(
         MameDataType::all_variants()
             .iter()
             .map(|&data_type| {
                 let progress_bar = multi_progress.add(ProgressBar::new(100));
                 progress_bar.set_style(
                     ProgressStyle::default_bar()
                         .template(&format!("{{spinner:.green}} [{{elapsed_precise}}] [{{bar:20.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{eta}}) {{msg}}"))
                         .progress_chars("#>-"),
                 );
                 (data_type, progress_bar)
             })
             .collect::<Vec<_>>(),
     );

    let shared_progress_callback: SharedProgressCallback = Arc::new(
        move |data_type: MameDataType, progress_info: ProgressInfo| {
            if let Some((_, progress_bar)) = progress_bars.iter().find(|(dt, _)| *dt == data_type) {
                // Update the progress bar
                match progress_info.callback_type {
                    CallbackType::Progress => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                    }
                    CallbackType::Info => {
                        progress_bar.set_message(progress_info.message);
                    }
                    CallbackType::Finish => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                        progress_bar.finish_with_message(progress_info.message);
                    }
                    CallbackType::Error => {
                        progress_bar.finish_with_message(progress_info.message);
                    }
                }
            }
        },
    );

    // Download the files
    let handles =
        mame_parser::file_handling::download_files(workspace_path, shared_progress_callback);

    // Wait for all threads to finish
    multi_progress.join().unwrap();

    // Print the result
    for handle in handles {
        match handle.join().unwrap() {
            Ok(_) => {
                // println!("Downloaded file: {}", path.display());
            }
            Err(e) => {
                eprintln!("Error during download: {}", e);
            }
        }
    }
    println!();
    Ok(())
}

fn unpack_all_files() -> Result<(), Box<dyn Error>> {
    show_section("Extract Files");

    // Define the workspace path
    let workspace_path = Path::new("data");

    // Create a multi progress bar
    let multi_progress = MultiProgress::new();

    // Create progress bars for each data type
    let progress_bars = Arc::new(
          MameDataType::all_variants()
              .iter()
              .map(|&data_type| {
                  let progress_bar = multi_progress.add(ProgressBar::new(100));
                  progress_bar.set_style(
                      ProgressStyle::default_bar()
                          .template(&format!("{{spinner:.green}} [{{elapsed_precise}}] [{{bar:20.cyan/blue}}] {{pos}}/{{len}} ({{eta}}) {{msg}}"))
                          .progress_chars("#>-"),
                  );
                  (data_type, progress_bar)
              })
              .collect::<Vec<_>>(),
      );

    let shared_progress_callback: SharedProgressCallback = Arc::new(
        move |data_type: MameDataType, progress_info: ProgressInfo| {
            if let Some((_, progress_bar)) = progress_bars.iter().find(|(dt, _)| *dt == data_type) {
                // Update the progress bar
                match progress_info.callback_type {
                    CallbackType::Progress => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                    }
                    CallbackType::Info => {
                        progress_bar.set_message(progress_info.message);
                    }
                    CallbackType::Finish => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                        progress_bar.finish_with_message(progress_info.message);
                    }
                    CallbackType::Error => {
                        progress_bar.finish_with_message(progress_info.message);
                    }
                }
            }
        },
    );

    // Unpack the files
    let handles =
        mame_parser::file_handling::unpack_files(workspace_path, shared_progress_callback);

    // Wait for all threads to finish
    multi_progress.join().unwrap();

    // Print the result
    for handle in handles {
        match handle.join().unwrap() {
            Ok(_) => {
                //println!("Unpacked data file: {}", path.display());
            }
            Err(e) => {
                eprintln!("Error during unpacking: {}", e);
            }
        }
    }

    println!();
    Ok(())
}

fn read_all_files() -> Result<(), Box<dyn Error>> {
    show_section("Read Files");

    // Define the workspace path
    let workspace_path = Path::new("data");

    // Create a multi progress bar
    let multi_progress: Arc<MultiProgress> = Arc::new(MultiProgress::new());

    // Create progress bars for each data type
    let progress_bars = Arc::new(
          MameDataType::all_variants()
              .iter()
              .map(|&data_type| {
                  let progress_bar = multi_progress.add(ProgressBar::new(100));
                  progress_bar.set_style(
                      ProgressStyle::default_bar()
                          .template(&format!("{{spinner:.green}} [{{elapsed_precise}}] [{{bar:20.cyan/blue}}] {{pos}}/{{len}} ({{eta}}) {{msg}}"))
                          .progress_chars("#>-"),
                  );
                  (data_type, progress_bar)
              })
              .collect::<Vec<_>>(),
      );

    // Create a shared progress callback
    let shared_progress_callback: SharedProgressCallback = Arc::new(
        move |data_type: MameDataType, progress_info: ProgressInfo| {
            if let Some((_, progress_bar)) = progress_bars.iter().find(|(dt, _)| *dt == data_type) {
                // Update the progress bar
                match progress_info.callback_type {
                    CallbackType::Progress => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                    }
                    CallbackType::Info => {
                        progress_bar.set_message(progress_info.message);
                    }
                    CallbackType::Finish => {
                        progress_bar.set_length(progress_info.total);
                        progress_bar.set_position(progress_info.progress);
                        progress_bar.finish_with_message(progress_info.message);
                    }
                    CallbackType::Error => {
                        progress_bar.finish_with_message(progress_info.message);
                    }
                }
            }
        },
    );

    let handle = thread::spawn(move || {
        multi_progress.join().unwrap();
    });

    // Read the files
    let machines = mame_parser::file_handling::read_files(workspace_path, shared_progress_callback);

    handle.join().unwrap();

    // Print the result
    match machines {
        Ok(machines) => {
            let mut machines_guard = MACHINES.lock().unwrap();
            *machines_guard = machines;
        }
        Err(e) => {
            eprintln!("Error reading data files: {}", e);
        }
    }

    println!();

    // Recreate the lists after reading the files.
    recreate_lists();

    Ok(())
}
