use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use crate::models::Machine;
use crate::readers::{
    catver_reader::read_catver_file, history_reader::read_history_file,
    languages_reader::read_languages_file, mame_reader::read_mame_file,
    nplayers_reader::read_nplayers_file, series_reader::read_series_file,
};

/**
 * Represents a data type that can be read by the application.
 */
pub struct DataType {
    pub name: &'static str,
    pub source: &'static str,
    pub source_match: &'static str,
    pub file_name_pattern: Regex,
    pub read_function:
        fn(&str, &mut HashMap<String, Machine>) -> Result<(), Box<dyn std::error::Error>>,
}

lazy_static! {
    /**
     * List of data types that can be read by the application.
     */
    pub static ref DATA_TYPES: Vec<DataType> = vec![
        DataType {
            name: "Mame",
            source: "https://www.progettosnaps.net/dats/MAME",
            source_match: "download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats",
            file_name_pattern: Regex::new(r"MAME\s+[0-9]*\.[0-9]+\.dat").unwrap(),
            read_function: read_mame_file,
        },
        DataType {
            name: "Languages",
            source: "https://www.progettosnaps.net/languages",
            source_match: "download",
            file_name_pattern: Regex::new(r"languages.ini").unwrap(),
            read_function: read_languages_file,
        },
        DataType {
            name: "NPlayers",
            source: "http://nplayers.arcadebelgium.be",
            source_match: "files",
            file_name_pattern: Regex::new(r"nplayers.ini").unwrap(),
            read_function: read_nplayers_file,
        },
        DataType {
            name: "Catver",
            source: "https://www.progettosnaps.net/catver",
            source_match: "download",
            file_name_pattern: Regex::new(r"catver.ini").unwrap(),
            read_function: read_catver_file,
        },
        DataType {
            name: "Series",
            source: "https://www.progettosnaps.net/series",
            source_match: "download",
            file_name_pattern: Regex::new(r"series.ini").unwrap(),
            read_function: read_series_file,
        },
        DataType {
            name: "History",
            source: "https://www.arcade-history.com/index.php?page=download",
            source_match: "dats",
            file_name_pattern: Regex::new(r"history.xml").unwrap(),
            read_function: read_history_file,
        },
    ];
}
