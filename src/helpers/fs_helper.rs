use std::{error::Error, fs, path::Path};

pub struct Paths {
    pub data_path: &'static str,
    pub download_path: &'static str,
    pub extract_path: &'static str,
    pub export_path: &'static str,
}

pub const PATHS: Paths = Paths {
    data_path: "data/",
    download_path: "data/downloads/",
    extract_path: "data/extracted/",
    export_path: "data/export",
};

/**
 * Check if the required folder structure exists and create it if it doesn't.
 */
pub fn check_folder_structure() -> Result<(), Box<dyn Error>> {
    let paths = [
        PATHS.data_path,
        PATHS.download_path,
        PATHS.extract_path,
        PATHS.export_path,
    ];

    let export_paths = [
        format!("{}/{}", PATHS.export_path, "csv"),
        format!("{}/{}", PATHS.export_path, "json"),
        format!("{}/{}", PATHS.export_path, "sqlite"),
    ];

    for path in paths.iter() {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
    }

    for path in export_paths.iter() {
        if !Path::new(path).exists() {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}
