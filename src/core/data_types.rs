use crate::core::readers::{
    catver_reader, history_reader, languages_reader, mame_reader, nplayers_reader,
    resources_reader, series_reader,
};
use lazy_static::lazy_static;
use regex::Regex;

/**
 * Represents a data type that can be read by the application.
 */
pub struct DataType {
    pub name: &'static str,
    pub source: &'static str,
    pub source_match: &'static str,
    pub zip_file_pattern: Regex,
    pub data_file_pattern: Regex,
    pub read_function: fn(&str) -> Result<(), Box<dyn std::error::Error>>,
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
            zip_file_pattern: Regex::new(r"^MAME_Dats_\d+\.7z$").unwrap(),
            data_file_pattern: Regex::new(r"MAME\s+[0-9]*\.[0-9]+\.dat").unwrap(),
            read_function: mame_reader::read_mame_file,
        },
        DataType {
            name: "Languages",
            source: "https://www.progettosnaps.net/languages",
            source_match: "download",
            zip_file_pattern: Regex::new(r"^pS_Languages_\d+\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"languages.ini").unwrap(),
            read_function: languages_reader::read_languages_file,
        },
        DataType {
            name: "NPlayers",
            source: "http://nplayers.arcadebelgium.be",
            source_match: "files",
            zip_file_pattern: Regex::new(r"^nplayers0\d+\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"nplayers.ini").unwrap(),
            read_function: nplayers_reader::read_nplayers_file,
        },
        DataType {
            name: "Catver",
            source: "https://www.progettosnaps.net/catver",
            source_match: "download",
            zip_file_pattern: Regex::new(r"^pS_CatVer_\d+\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"catver.ini").unwrap(),
            read_function: catver_reader::read_catver_file,
        },
        DataType {
            name: "Series",
            source: "https://www.progettosnaps.net/series",
            source_match: "download",
            zip_file_pattern: Regex::new(r"^pS_Series_\d+\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"series.ini").unwrap(),
            read_function: series_reader::read_series_file,
        },
        DataType {
            name: "History",
            source: "https://www.arcade-history.com/index.php?page=download",
            source_match: "dats",
            zip_file_pattern: Regex::new(r"^history\d+\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"history.xml").unwrap(),
            read_function: history_reader::read_history_file,
        },
        DataType {
            name: "Resources",
            source: "https://www.progettosnaps.net/dats",
            source_match: "download/?tipo=dat_resource&file=/dats/cmdats/pS_AllProject_",
            zip_file_pattern: Regex::new(r"^pS_AllProject_\d{8}_\d+_\([a-zA-Z]+\)\.zip$").unwrap(),
            data_file_pattern: Regex::new(r"^pS_AllProject_\d{8}_\d+_\([a-zA-Z]+\)\.dat$").unwrap(),
            read_function: resources_reader::read_resources_file,
        }
    ];
}
