use regex::Regex;
use lazy_static::lazy_static;

pub struct DataType {
    pub name: &'static str,
    pub source: &'static str,
    pub source_match: &'static str,
    pub file_name_pattern: Regex,
}

lazy_static! {
    pub static ref DATA_TYPES: Vec<DataType> = vec![
        DataType {
            name: "Mame",
            source: "https://www.progettosnaps.net/dats/MAME",
            source_match: "download/?tipo=dat_mame&file=/dats/MAME/packs/MAME_Dats",
            file_name_pattern: Regex::new(r"MAME\s+[0-9]*\.[0-9]+\.dat").unwrap(),
        },
        DataType {
            name: "Languages",
            source: "https://www.progettosnaps.net/languages",
            source_match: "download",
            file_name_pattern: Regex::new(r"languages.ini").unwrap(),
        },
        DataType {
            name: "NPlayers",
            source: "http://nplayers.arcadebelgium.be",
            source_match: "files",
            file_name_pattern: Regex::new(r"nplayers.ini").unwrap(),
        },
        DataType {
            name: "Catver",
            source: "https://www.progettosnaps.net/catver",
            source_match: "download",
            file_name_pattern: Regex::new(r"catver.ini").unwrap(),
        },
        DataType {
            name: "Series",
            source: "https://www.progettosnaps.net/series",
            source_match: "download",
            file_name_pattern: Regex::new(r"series.ini").unwrap(),
        },
        DataType {
            name: "History",
            source: "https://www.arcade-history.com/index.php?page=download",
            source_match: "dats",
            file_name_pattern: Regex::new(r"history.xml").unwrap(),
        },
    ];
}
