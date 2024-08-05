use crate::core::models::Machine;
use serde_json::to_string;
use std::io::Write;
use std::{
    collections::HashMap,
    fs::File,
    sync::{Arc, Mutex},
};

/**
 * Write the given machines data to a json file.
 */
pub fn write_machines(
    db_path: &str,
    machines: Arc<Mutex<HashMap<String, Machine>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_string(&*machines)?;
    let mut file = File::create(db_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
