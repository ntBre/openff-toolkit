use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    qcportal::models::TorsionDriveRecord, smirnoff::ForceField,
    topology::molecule::Molecule,
};

use self::filters::Filters;

pub mod filters;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Provenance {
    #[serde(rename = "applied-filters", default)]
    pub applied_filters: Filters,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Entry {
    #[serde(rename = "type")]
    pub typ: String,

    pub record_id: String,

    pub cmiles: String,

    pub inchi_key: String,
}

impl Entry {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TorsionDriveResultCollection {
    pub entries: HashMap<String, Vec<Entry>>,
    pub provenance: Provenance,

    #[serde(rename = "type")]
    pub typ: String,
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
                        conformers: Vec::new(),
                    },
                ));
            }
        }
        ret
    }

    /// in the common case where there is only a single entry in the dataset,
    /// return a reference to the vector of entries. If there are multiple
    /// entries, return `Err(())`
    pub fn entries(&self) -> Option<&[Entry]> {
        if self.entries.len() == 1 {
            return Some(self.entries.iter().next().unwrap().1);
        }
        None
    }

    /// return the sum of lengths of `self.entries`
    pub fn len(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// TODO is there more to this? it's a straight copy paste right now? even worth
// having?
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OptimizationResultCollection {
    pub entries: HashMap<String, Vec<Entry>>,
    pub provenance: Provenance,

    #[serde(rename = "type")]
    pub typ: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_filtered() {
        let got = TorsionDriveResultCollection::parse_file(
            "../../projects/valence-fitting/02_curate-data/datasets/\
	     filtered-sage-td.json",
        )
        .unwrap();

        insta::assert_debug_snapshot!(
            serde_json::to_string_pretty(&got).unwrap()
        );
    }
}
