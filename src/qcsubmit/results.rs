use std::{collections::HashMap, error::Error, fs::read_to_string, path::Path};

use serde::{Deserialize, Serialize};

use crate::{qcportal::models::Record, topology::molecule::Molecule};

use self::filters::Filters;

use super::client::FractalClient;

pub mod filters;

#[cfg(test)]
mod tests;

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
pub struct ResultCollection {
    pub entries: HashMap<String, Vec<Entry>>,
    pub provenance: Provenance,

    #[serde(rename = "type")]
    pub typ: String,
}

impl ResultCollection {
    pub fn parse_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(path)?;
        let ret = serde_json::from_str(&contents)?;
        Ok(ret)
    }

    // TODO this is supposed to have all kinds of wacky caching stuff and
    // probably actually retrieving from qcarchive. for now just return what's
    // in the hashmap, which I think should be correct for
    // `TorsionDriveResultCollection`s from files
    pub fn to_records(self) -> Vec<(Record, Molecule)> {
        let mut ret = Vec::new();
        let client = FractalClient::new();
        let results = client.optimization_records(self, 400);
        for (record, cmiles, geom) in results {
            let mut molecule = Molecule::from_mapped_smiles(cmiles, true);
            // TODO really taking here, shouldn't need clone
            molecule.add_conformer(geom[0].clone());
            ret.push((Record { id: record.id }, molecule));
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
