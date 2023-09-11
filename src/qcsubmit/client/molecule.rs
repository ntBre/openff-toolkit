use super::Body;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize)]
struct QueryFilter {
    limit: Option<usize>,
    skip: usize,
}

#[derive(Serialize)]
struct Data {
    id: Vec<String>,
    molecule_hash: Option<()>,
    molecular_formula: Option<()>,
}

#[derive(Serialize)]
pub struct MoleculeGetBody {
    meta: QueryFilter,
    data: Data,
}

impl Body for MoleculeGetBody {
    fn new(id: Vec<String>) -> Self {
        Self {
            meta: QueryFilter::default(),
            data: Data {
                id,
                molecule_hash: None,
                molecular_formula: None,
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Identifiers {
    pub molecule_hash: String,
    pub molecular_formula: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Molecule {
    pub symbols: Vec<String>,
    pub geometry: Vec<f64>,
    pub name: String,
    pub identifiers: Identifiers,

    /// looks like an int in my test cases, but the
    /// qcelemental/models/molecule.py says float
    pub molecular_charge: f64,

    pub molecular_multiplicity: usize,

    /// tuple of `(atom_index1, atom_index2, bond_order)`
    pub connectivity: Vec<(usize, usize, f64)>,

    pub fix_com: bool,
    pub fix_orientation: bool,
    pub fix_symmetry: String,
    pub id: String,
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::qcsubmit::client::procedure::Response;

    use super::*;

    #[test]
    fn de_molecule() {
        let s = read_to_string("testfiles/molecules.json").unwrap();
        let _: Response<Molecule> = serde_json::from_str(&s).unwrap();
    }
}
