use crate::models::{Machine};
use std::{collections::HashMap, error::Error};

pub fn read_catver_file(file_path: &str, machines: &mut HashMap<String, Machine>) -> Result<(), Box<dyn Error>>{
    _ = file_path;
    _ = machines;
    Ok(())
}