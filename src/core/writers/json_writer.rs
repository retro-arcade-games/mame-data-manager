use crate::core::data::MACHINES;
use serde_json::to_string;
use std::fs::File;
use std::io::Write;

/**
 * Write the given machines data to a json file.
 */
pub fn write_machines(db_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let machines = MACHINES.lock().unwrap();
    let json = to_string(&*machines)?;
    let mut file = File::create(db_path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
