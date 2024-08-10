use crate::{
    core::{data::MACHINES, models::CustomData},
    helpers::ui_helper::init_progress_bar,
};
use std::error::Error;

/**
 * Refactor the names of the machines to ensure consistency and correctness.
 */
pub fn refactor_names() -> Result<(), Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();
    let pb = init_progress_bar(machines.len() as u64, "machines in collection");

    let mut processed_count = 0;
    let batch = 5000;

    // Iterate the machines hashmap
    for (_, machine) in machines.iter_mut() {
        // Refactor the machine name
        let refactored_name = refactor_name(&machine.description);
        // Assign the refactored name to the machine in custom data
        if machine.custom_data.is_none() {
            machine.custom_data = Some(CustomData::default());
        }
        machine.custom_data.as_mut().unwrap().name = Some(refactored_name.clone());
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

fn refactor_name(description: &Option<String>) -> String {
    if description.is_none() {
        return String::new();
    }

    let step1 = description
        .as_ref()
        .unwrap()
        .replace('?', "")
        .replace("&amp;", "&");
    let step2: String = step1.split('(').next().unwrap_or("").to_string();

    // Paso 3: Capitalizar la primera letra de cada palabra
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in step2.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
