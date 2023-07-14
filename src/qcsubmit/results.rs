use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::{
    qcportal::models::TorsionDriveRecord, smirnoff::ForceField,
    topology::molecule::Molecule,
};

#[derive(Clone, Debug, Deserialize)]
struct Entry {
    #[serde(rename = "type")]
    typ: String,

    record_id: String,

    cmiles: String,

    inchi_key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TorsionDriveResultCollection {
    entries: HashMap<String, Vec<Entry>>,
}

impl TorsionDriveResultCollection {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let ret = serde_json::from_str(&contents)?;
        Ok(ret)
    }

    // TODO this is supposed to have all kinds of wacky caching stuff and
    // probably actually retrieving from qcarchive. for now just return what's
    // in the hashmap, which I think should be correct for
    // `TorsionDriveResultCollection`s from files
    pub fn to_records(self) -> Vec<(TorsionDriveRecord, Molecule)> {
        let mut ret = Vec::new();
        for (_client_address, entries) in self.entries {
            for entry in entries {
                ret.push((
                    TorsionDriveRecord {
                        id: entry.record_id,
                    },
                    // TODO convert cmiles to smiles
                    Molecule {
                        atoms: Vec::new(),
                        name: String::new(),
                        smiles: entry.cmiles,
                    },
                ));
            }
        }
        ret
    }
}
