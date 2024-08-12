use crate::core::models::Machine;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    // HashMap that stores the machines in the system.
    pub static ref MACHINES: Arc<Mutex<HashMap<String, Machine>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the series.
    pub static ref SERIES: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the manufacturers.
    pub static ref MANUFACTURERS: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the players.
    pub static ref PLAYERS: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the languages.
    pub static ref LANGUAGES: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the categories.
    pub static ref CATEGORIES: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    // HashMap that stores the subcategories.
    pub static ref SUBCATEGORIES: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));
}

/**
 * Add an item to the list.
 */
pub fn add_item_to_list(map: &Arc<Mutex<HashMap<String, usize>>>, name: String) {
    let mut locked_map = map.lock().unwrap();
    let counter = locked_map.entry(name).or_insert(0);
    *counter += 1;
}

/**
 * Get the list of items.
 */
pub fn get_list(map: &Arc<Mutex<HashMap<String, usize>>>) -> Vec<String> {
    let locked_map = map.lock().unwrap();
    let mut list: Vec<String> = locked_map.keys().cloned().collect();
    list.sort();
    list
}

/**
 * Clear the list.
 */
pub fn clear_list(map: &Arc<Mutex<HashMap<String, usize>>>) {
    let mut locked_map = map.lock().unwrap();
    locked_map.clear();
}

/**
 * Get the top items from the list.
 */
pub fn get_top(map: &Arc<Mutex<HashMap<String, usize>>>, count: usize) -> Vec<(String, usize)> {
    let locked_map = map.lock().unwrap();
    let mut vec: Vec<_> = locked_map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));
    vec.into_iter()
        .take(count)
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

/**
 * Recreate the lists.
 */
pub fn recreate_lists() {
    let mut machines = MACHINES.lock().unwrap();

    clear_list(&SERIES);
    create_series_list(&mut machines);

    clear_list(&MANUFACTURERS);
    create_manufacturers_list(&mut machines);

    clear_list(&PLAYERS);
    create_players_list(&mut machines);

    clear_list(&LANGUAGES);
    create_languages_list(&mut machines);
}

/**
 * Create a list of series from the given HashMap of machines.
 */
fn create_series_list(machines: &mut HashMap<String, crate::core::models::Machine>) {
    for (_, machine) in machines.iter() {
        if let Some(series_name) = &machine.series {
            add_item_to_list(&SERIES, series_name.clone())
        }
    }
}

/**
 * Create a list of unique manufacturers from the machines in the system.
 */
fn create_manufacturers_list(machines: &HashMap<String, Machine>) {
    for (_, machine) in machines.iter() {
        if let Some(extended_data) = &machine.extended_data {
            if let Some(manufacturer) = &extended_data.manufacturer {
                add_item_to_list(&MANUFACTURERS, manufacturer.clone());
            }
        }
    }
}

/**
 * Create a list of unique players from the machines in the system.
 */
fn create_players_list(machines: &HashMap<String, Machine>) {
    for (_, machine) in machines.iter() {
        if let Some(extended_data) = &machine.extended_data {
            if let Some(players) = &extended_data.players {
                let players = players.split(',').map(|s| s.trim()).collect::<Vec<&str>>();
                for player in players {
                    add_item_to_list(&PLAYERS, player.to_string());
                }
            }
        }
    }
}

/**
 * Create a list of unique languages from the machines in the system.
 */
fn create_languages_list(machines: &HashMap<String, Machine>) {
    for (_, machine) in machines.iter() {
        for language in &machine.languages {
            add_item_to_list(&LANGUAGES, language.clone());
        }
    }
}
