pub struct Paths {
    pub data_path: &'static str,
    pub download_path: &'static str,
    pub extracted_path: &'static str,
}

pub const PATHS: Paths = Paths {
    data_path: "data/",
    download_path: "data/downloads/",
    extracted_path: "data/extracted/",
};

/**
 * Find a file with the given pattern in the given folder.
 */
pub fn find_file_with_pattern(folder: &str, pattern: &regex::Regex) -> Option<String> {
    for entry in walkdir::WalkDir::new(folder)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                if pattern.is_match(file_name) {
                    // return Some(path.to_path_buf());
                    return Some(path.to_string_lossy().into_owned());
                }
            }
        }
    }
    None
}

/**
 * Get the file name from the given URL.
 */
pub fn get_file_name(url: &str) -> String {
    let last_param = url.split('/').last().unwrap_or("");
    let file_name = last_param.split('=').last().unwrap_or("");
    file_name.to_string()
}
