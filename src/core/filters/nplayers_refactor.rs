use crate::{
    core::{data::MACHINES, models::CustomData},
    helpers::ui_helper::init_progress_bar,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, error::Error};

lazy_static! {
    static ref SUBSTITUTIONS: HashMap<&'static str, &'static str> = {
        let substitutions = vec![
            ("1P", "Single-player game"),
            ("2P alt", "Alternate two-player mode"),
            ("2P sim", "Simultaneous two-player mode"),
            ("3P alt", "Alternate three-player mode"),
            ("3P sim", "Simultaneous three-player mode"),
            ("4P alt", "Alternate four-player mode"),
            ("4P sim", "Simultaneous four-player mode"),
            ("5P alt", "Alternate five-player mode"),
            ("6P alt", "Alternate six-player mode"),
            ("6P sim", "Simultaneous six-player mode"),
            ("8P alt", "Alternate eight-player mode"),
            ("8P sim", "Simultaneous eight-player mode"),
            ("9P alt", "Alternate nine-player mode"),
            ("???", "Unknown or unspecified number of players"),
            ("BIOS", "BIOS"),
            ("Device", "Non-playable device"),
            ("Non-arcade", "Non-arcade game"),
        ];
        substitutions.into_iter().collect()
    };
}

/**
 * Refactor the number of players field.
 */
pub fn refactor_nplayers() -> Result<(), Box<dyn Error>> {
    let mut machines = MACHINES.lock().unwrap();
    let pb = init_progress_bar(machines.len() as u64, "machines in collection");

    let mut processed_count = 0;
    let batch = 5000;

    for (_, machine) in machines.iter_mut() {
        let refactored_name = refactor_nplayer(&machine.players);
        if machine.custom_data.is_none() {
            machine.custom_data = Some(CustomData::default());
        }
        machine.custom_data.as_mut().unwrap().players = Some(refactored_name.clone());
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
 * Refactor the number of players field.
 */
fn refactor_nplayer(nplayers: &Option<String>) -> String {
    nplayers
        .as_ref()
        .unwrap_or(&"Unknown".to_string())
        .split('/')
        .map(|part| {
            let part = part.trim();
            SUBSTITUTIONS.get(part).unwrap_or(&part).to_string()
        })
        .collect::<Vec<_>>()
        .join(", ")
}
