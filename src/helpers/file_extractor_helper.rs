use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use zip::read::ZipArchive;
use sevenz_rust::decompress_file;

/**
 * Extract the contents of the given archive file to the given destination folder.
 */
pub fn extract_file(archive_path: &str, destination_folder: &str) -> Result<(), Box<dyn Error>> {
    if archive_path.ends_with(".zip") {
        extract_zip(archive_path, destination_folder)?;
    } else if archive_path.ends_with(".7z") {
        extract_7zip(archive_path, destination_folder)?;
    } else {
        return Err("Unsupported archive format".into());
    }
    Ok(())
}

/**
 * Extract the contents of a ZIP archive to the given destination folder.
 */
fn extract_zip(archive_path: &str, destination_folder: &str) -> Result<(), Box<dyn Error>> {
    
    let file = File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    
    let pb = ProgressBar::new(archive.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .progress_chars("#>-"),
    );

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(destination_folder).join(file.name());

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        pb.inc(1);
    }

    pb.finish_and_clear();

    Ok(())
}

/**
 * Extract the contents of a 7zip archive to the given destination folder.
 */
fn extract_7zip(archive_path: &str, destination_folder: &str) -> Result<(), Box<dyn Error>> {
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .tick_strings(&["|", "/", "-", "\\"]),
    );

    pb.set_message("Extracting 7zip archive...");

    decompress_file(archive_path, destination_folder)?;

    pb.finish_and_clear();

    Ok(())
}
