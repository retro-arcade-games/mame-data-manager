use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Machine {
    pub name: String,
    pub source_file: Option<String>,
    pub rom_of: Option<String>,
    pub clone_of: Option<String>,
    pub is_bios: Option<bool>,
    pub is_device: Option<bool>,
    pub runnable: Option<bool>,
    pub is_mechanical: bool,
    pub sample_of: Option<String>,
    pub description: Option<String>,
    pub year: Option<String>,
    pub manufacturer: Option<String>,
    pub bios_sets: Vec<BiosSet>,
    pub roms: Vec<Rom>,
    pub device_refs: Vec<DeviceRef>,
    pub software_list: Vec<Software>,
    pub samples: Vec<Sample>,
    pub driver_status: Option<String>,
    pub languages: Vec<Language>,
    pub players: Option<String>,
    pub series: Option<String>,
    pub genre: Option<String>,
    pub subgenre: Option<String>,
    pub is_mature: Option<bool>,
    pub history_sections: Vec<HistorySection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiosSet {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rom {
    pub name: String,
    pub size: u64,
    pub merge: Option<String>,
    pub status: Option<String>,
    pub crc: Option<String>,
    pub sha1: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRef {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Software {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorySection {
    pub name: String,
    pub text: String,
    pub order: u32,
}
