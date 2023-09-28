use std::{clone::Clone, collections::HashMap};

use pyo3::{types::PyModule, FromPyObject, PyResult, Python};
use torx::Graph;

use crate::qcsubmit::client::Cmiles;

use super::{ChemicalEnvironmentMatch, Topology};

/// extract a simple field name from a Python object
#[inline]
fn extractor<'source, T: FromPyObject<'source>>(
    ob: &'source pyo3::PyAny,
    field: &'static str,
) -> T {
    ob.getattr(field).unwrap().extract::<T>().unwrap()
}

fn extract_conformers<'source>(
    ob: &'source pyo3::PyAny,
) -> PyResult<Vec<Vec<f64>>> {
    Python::with_gil(|py| {
        let flatten = PyModule::from_code(
            py,
            "def flatten(array):
    import numpy as np
    return [a.magnitude.flatten().tolist() for a in array]",
            "",
            "",
        )
        .unwrap()
        .getattr("flatten")
        .unwrap();
        Ok(flatten.call1((ob,)).unwrap().extract().unwrap())
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum Stereochemistry {
    R,
    S,
    None,
}

impl<'source> FromPyObject<'source> for Stereochemistry {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        match ob.getattr("stereochemistry") {
            Ok(_) => todo!(),
            Err(_) => Ok(Stereochemistry::None),
        }
    }
}

#[derive(Clone, PartialEq, FromPyObject)]
pub struct Molecule {
    pub name: String,
    #[pyo3(from_py_with = "extract_conformers")]
    pub conformers: Vec<Vec<f64>>,
    #[pyo3(attribute("atoms"))]
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bond>,
}

#[cfg(feature = "rodeo")]
impl From<rodeo::RWMol> for Molecule {
    fn from(_value: rodeo::RWMol) -> Self {
        todo!()
    }
}

impl From<Molecule> for Graph {
    fn from(value: Molecule) -> Self {
        let mut g = Graph::new();
        for atom in value.atoms {
            g.add_node(atom.molecule_atom_index.unwrap());
        }

        for bond in value.bonds {
            g.add_edge(bond.atom1_index, bond.atom2_index);
        }
        g
    }
}

impl Molecule {
    /// load an SDF file
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Self {
        let mol = ligand::molecule::Molecule::from_file(file).unwrap();
        Python::with_gil(|py| mol.inner.extract(py).unwrap())
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

    pub(crate) fn chemical_environment_matches(
        &self,
        _smarts: &str,
    ) -> Vec<ChemicalEnvironmentMatch> {
        todo!()
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

/// A chemical atom.
#[derive(Clone, PartialEq)]
pub struct Atom {
    pub atomic_number: usize,
    formal_charge: isize,
    is_aromatic: bool,
    name: Option<String>,
    stereochemistry: Stereochemistry,

    // computed properties
    bonds: Vec<Bond>,
    molecule_atom_index: Option<usize>,
}

impl<'source> FromPyObject<'source> for Atom {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        let atomic_number = extractor(ob, "atomic_number");
        // quantity-wrapped but unit is elementary charge
        let formal_charge =
            extractor(ob.getattr("formal_charge").unwrap(), "magnitude");
        let is_aromatic = extractor(ob, "is_aromatic");
        Ok(Self {
            atomic_number,
            formal_charge,
            is_aromatic,
            name: extractor(ob, "name"),
            molecule_atom_index: extractor(ob, "molecule_atom_index"),
            bonds: extractor(ob, "bonds"),
            stereochemistry: extractor(ob, "stereochemistry"),
        })
    }
}

#[allow(unused)]
impl Atom {
    fn new(
        atomic_number: usize,
        formal_charge: isize,
        is_aromatic: bool,
        name: Option<String>,
        stereochemistry: Stereochemistry,
    ) -> Self {
        Self {
            atomic_number,
            formal_charge,
            is_aromatic,
            name,
            stereochemistry,
            bonds: Vec::new(),
            molecule_atom_index: None,
        }
    }

    fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond);
    }

    fn formal_charge(&self) -> isize {
        self.formal_charge
    }

    fn set_formal_charge(&mut self, charge: isize) {
        self.formal_charge = charge;
    }

    fn partial_charge(&self) -> isize {
        todo!();
    }

    fn set_partial_charge(&mut self, _charge: isize) {
        todo!();
    }

    fn is_aromatic(&self) -> bool {
        self.is_aromatic
    }

    fn stereochemistry(&self) -> Stereochemistry {
        self.stereochemistry
    }

    // // NOTE below pasted in from python
    fn set_stereochemistry(&mut self, stereochemistry: Stereochemistry) {
        self.stereochemistry = stereochemistry;
    }

    fn atomic_number(&self) -> usize {
        self.atomic_number
    }

    fn symbol(&self) -> &'static str {
        // TODO put this in openff-units/elements
        const SYMBOLS: [&str; 117] = [
            "X", "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na",
            "Mg", "Al", "Si", "P", "S", "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V",
            "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As", "Se",
            "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh",
            "Pd", "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba",
            "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho",
            "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt",
            "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac",
            "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm",
            "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg",
            "Uub", "Uut", "Uuq", "Uup", "Uuh",
        ];
        SYMBOLS[self.atomic_number]
    }

    fn mass(&self) -> Quantity {
        #[rustfmt::skip]
        const MASSES: [f64; 117] = [
            0.0, 1.007947, 4.003, 6.9412, 9.0121823, 10.8117, 12.01078,
            14.00672, 15.99943, 18.99840325, 20.17976, 22.989769282,
            24.30506, 26.98153868, 28.08553, 30.9737622, 32.0655,
            35.4532, 39.9481, 39.09831, 40.0784, 44.9559126, 47.8671,
            50.94151, 51.99616, 54.9380455, 55.8452, 58.9331955,
            58.69342, 63.5463, 65.4094, 69.7231, 72.641, 74.921602,
            78.963, 79.9041, 83.7982, 85.46783, 87.621, 88.905852,
            91.2242, 92.906382, 95.942, 98.0, 101.072, 102.905502,
            106.421, 107.86822, 112.4118, 114.8183, 118.7107, 121.7601,
            127.603, 126.904473, 131.2936, 132.90545192, 137.3277,
            138.905477, 140.1161, 140.907652, 144.2423, 145.0,
            150.362, 151.9641, 157.253, 158.925352, 162.5001,
            164.930322, 167.2593, 168.934212, 173.043, 174.9671,
            178.492, 180.947882, 183.841, 186.2071, 190.233,
            192.2173, 195.0849, 196.9665694, 200.592, 204.38332,
            207.21, 208.980401, 209.0, 210.0, 222.018, 223.0, 226.0,
            227.0, 232.038062, 231.035882, 238.028913, 237.0, 244.0,
            243.0, 247.0, 247.0, 251.0, 252.0, 257.0, 258.0, 259.0,
            262.0, 261.0, 262.0, 266.0, 264.0, 269.0, 268.0, 281.0,
            272.0, 285.0, 284.0, 289.0, 288.0, 292.0,
        ];
        Quantity {
            value: MASSES[self.atomic_number],
            unit: Unit::Dalton,
        }
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    fn bonds(&self) -> &Vec<Bond> {
        &self.bonds
    }

    /// TODO potentially difficult. calls out to
    /// openff.toolkit.utils.toolkits.ToolkitRegistry to compute
    fn is_in_ring(&self) -> bool {
        todo!()
    }
}

#[derive(Clone, PartialEq, FromPyObject)]
pub struct Bond {
    atom1_index: usize,
    atom2_index: usize,
    bond_order: usize,
    is_aromatic: bool,
    fractional_bond_order: Option<f64>,
    stereochemistry: Stereochemistry,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file() {
        Molecule::from_file(
            "/home/brent/omsf/rust/anakin/targets/torsion-18535805/input.sdf",
        );
    }
}
