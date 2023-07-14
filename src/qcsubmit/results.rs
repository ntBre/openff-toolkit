use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    #[serde(rename = "type")]
    typ: String,

    record_id: String,

    cmiles: String,

    inchi_key: String,
}

#[derive(Debug, Deserialize)]
pub struct TorsionDriveResultCollection {
    entries: HashMap<String, Vec<Entry>>,
}

impl TorsionDriveResultCollection {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let ret = serde_json::from_str(&contents)?;
        Ok(ret)
    }
}
