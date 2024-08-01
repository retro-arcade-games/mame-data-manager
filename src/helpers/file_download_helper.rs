use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

/**
 * Download a file from the given URL and save it to the given file path.
 */
pub fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let response = Client::new().get(url).send()?;
    let total_size = response.content_length();

    let pb = if let Some(size) = total_size {
        let pb = ProgressBar::new(size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
                )
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let mut file = File::create(file_path)?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0; 4096];

    let mut response = response;
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        if let Some(pb) = &pb {
            pb.set_position(downloaded);
        }
    }

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    Ok(())
}
