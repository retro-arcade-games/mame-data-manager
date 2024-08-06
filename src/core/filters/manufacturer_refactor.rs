use crate::{
    core::models::{CustomData, Machine},
    helpers::ui_helper::init_progress_bar,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, error::Error};

lazy_static! {
    static ref RE_COMMON: Regex = Regex::new(r"(?i)\b(Games|Corp|Inc|Ltd|Co|Corporation|Industries|Elc|S\.R\.L|S\.A|inc|of America|Japan|UK|USA|Europe|do Brasil|du Canada|Canada|America|Austria|of)\b\.?").unwrap();
    static ref RE_PUNCTUATION: Regex = Regex::new(r"[.,?]+$|-$").unwrap();
    static ref NEEDS_CLEANING: Regex = Regex::new(r"[\(/,?]|(Games|Corp|Inc|Ltd|Co|Corporation|Industries|Elc|S\.R\.L|S\.A|inc|of America|Japan|UK|USA|Europe|do Brasil|du Canada|Canada|America|Austria|of)").unwrap();
}

/**
 * Refactor the manufacturer name.
 */
pub fn refactor_manufacturers(
    machines: &mut HashMap<String, Machine>,
) -> Result<(), Box<dyn Error>> {
    let pb = init_progress_bar(machines.len() as u64, "machines in collection");

    let mut processed_count = 0;
    let batch = 500;

    // Iterate the machines hashmap
    for (_, machine) in machines.iter_mut() {
        // Refactor the machine name
        let refactored_manufacturer = refactor_manufacturer(&machine.manufacturer);
        // Assign the refactored name to the machine in custom data
        if machine.custom_data.is_none() {
            machine.custom_data = Some(CustomData::default());
        }
        machine.custom_data.as_mut().unwrap().manufacturer = Some(refactored_manufacturer.clone());
        processed_count += 1;
        if processed_count % batch == 0 {
            pb.inc(batch);
        }
    }
    let remaining = processed_count % batch;
    if remaining > 0 {
        pb.inc(remaining as u64);
    }

    pb.finish_and_clear();

    Ok(())
}

/**
 * Refactor the manufacturer name.
 */
fn refactor_manufacturer(manufacturer: &Option<String>) -> String {
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