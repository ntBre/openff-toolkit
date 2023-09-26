use std::collections::HashMap;

use pyo3::{types::PyModule, Py, PyAny, Python};

use crate::qcsubmit::client::Cmiles;

use super::Topology;

#[derive(Clone, Copy, PartialEq)]
pub enum Stereochemistry {
    R,
    S,
    None,
}

#[derive(Clone)]
pub struct Molecule {
    #[allow(unused)]
    inner: Py<PyAny>,
}

#[cfg(feature = "rodeo")]
impl From<rdkit_wrapper::RWMol> for Molecule {
    fn from(rdmol: rdkit_wrapper::RWMol) -> Self {
        let inner = Python::with_gil(|py| {
            let tk = PyModule::import(py, "openff.toolkit").unwrap();
            let fun = tk.getattr("Molecule").unwrap();
            fun.call1((rdmol,)).unwrap().into()
        });
        Self { inner }
    }
}

impl Molecule {
    /// load an SDF file
    #[cfg(feature = "rodeo")]
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Self {
        let mut rdmol = rdkit_wrapper::RWMol::from_sdf(file).unwrap();
        use rdkit_wrapper::SanitizeOptions as S;
        rdmol.sanitize(S::All ^ S::SetAromaticity ^ S::AdjustHs);
        rdmol.assign_stereochemistry_from_3d();
        rdmol.set_aromaticity(rdkit_wrapper::AromaticityModel::MDL);
        rdmol.into()
    }

    pub fn from_mapped_smiles(
        _smiles: Cmiles,
        _allow_undefined_stereo: bool,
    ) -> Self {
        todo!()
    }

    pub fn to_topology(self) -> Topology {
        Topology::from_molecules(vec![self])
    }

    pub fn are_isomorphic(
        &self,
        _mol2: &Molecule,
    ) -> (bool, HashMap<usize, usize>) {
        todo!();
    }
}

#[allow(unused)]
enum Unit {
    Dalton,
}

#[allow(unused)]
struct Quantity {
    value: f64,
    unit: Unit,
}

/// Base trait for all particles in a molecule. TODO : Serialize
trait Particle {
    fn molecule(&self) -> Option<&Molecule>;

    /// Set the particle's molecule. panics if the particle already contains
    /// a molecule
    fn set_molecule(&mut self, molecule: Molecule);

    /// Returns the index of this particle in its molecule, if it has
    /// one
    fn molecule_particle_index(&self) -> Option<usize>;

    /// The name of the particle
    fn name(&self) -> Option<&String>;
}
