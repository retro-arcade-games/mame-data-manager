use crate::core::models::Machine;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    // This is a HashMap that stores the machines in the system.
    pub static ref MACHINES: Arc<Mutex<HashMap<String, Machine>>> = Arc::new(Mutex::new(HashMap::new()));
    // This is a Vec that stores the names of manufacturers.
    pub static ref MANUFACTURERS: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
}
