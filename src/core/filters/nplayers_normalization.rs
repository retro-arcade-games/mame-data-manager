use lazy_static::lazy_static;
use std::collections::HashMap;

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
 * Normalize the number of players.
 */
pub fn normalize_nplayer(nplayers: &Option<String>) -> String {
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
