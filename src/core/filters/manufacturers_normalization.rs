use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_COMMON: Regex = Regex::new(r"(?i)\b(Games|Corp|Inc|Ltd|Co|Corporation|Industries|Elc|S\.R\.L|S\.A|inc|of America|Japan|UK|USA|Europe|do Brasil|du Canada|Canada|America|Austria|of)\b\.?").unwrap();
    static ref RE_PUNCTUATION: Regex = Regex::new(r"[.,?]+$|-$").unwrap();
    static ref NEEDS_CLEANING: Regex = Regex::new(r"[\(/,?]|(Games|Corp|Inc|Ltd|Co|Corporation|Industries|Elc|S\.R\.L|S\.A|inc|of America|Japan|UK|USA|Europe|do Brasil|du Canada|Canada|America|Austria|of)").unwrap();
}

/**
 * Normalize the manufacturer name.
 */
pub fn normalize_manufacturer(manufacturer: &Option<String>) -> String {
    // Keep only the first part of the manufacturer removing anything after (, /
    let parts: Vec<&str> = manufacturer
        .as_ref()
        .unwrap()
        .split(&['(', '/'][..])
        .collect();
    let mut result = parts[0].to_string();

    // Check if needs cleaning
    if NEEDS_CLEANING.is_match(&result) {
        result = RE_COMMON.replace_all(&result, "").to_string();
        result = RE_PUNCTUATION.replace_all(&result, "").to_string();
    }

    result = result.replace('?', "").replace(',', "");
    result = result.replace("<unknown>", "Unknown");
    result = result.trim().to_string();

    result
}
