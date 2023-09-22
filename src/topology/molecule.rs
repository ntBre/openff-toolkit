use std::collections::HashMap;

use crate::qcsubmit::client::Cmiles;

use super::{ChemicalEnvironmentMatch, Topology};

#[derive(Clone, Copy, PartialEq)]
pub enum Stereochemistry {
    R,
    S,
    None,
}

#[derive(Clone, PartialEq)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub name: String,
    pub smiles: String,
    pub conformers: Vec<Vec<f64>>,
}

#[cfg(feature = "rodeo")]
impl From<rodeo::RWMol> for Molecule {
    fn from(_value: rodeo::RWMol) -> Self {
        todo!()
    }
}

impl Molecule {
    /// load an SDF file
    #[cfg(feature = "rodeo")]
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Self {
        let mut rdmol = rodeo::RWMol::from_sdf(file);
        use rodeo::SanitizeOptions as S;
        rdmol.sanitize(S::All ^ S::SetAromaticity ^ S::AdjustHs);
        rdmol.assign_stereochemistry_from_3d();
        rdmol.set_aromaticity(rodeo::AromaticityModel::MDL);
        rdmol.into()
    }

    pub fn from_mapped_smiles(
        _smiles: Cmiles,
        _allow_undefined_stereo: bool,
    ) -> Self {
        todo!()
    }

    fn index(&self, atom: &Atom) -> Option<usize> {
        self.atoms.iter().position(|a| a == atom)
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

type AtomMetadata = HashMap<String, String>;

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
    molecule: Option<Molecule>,
    stereochemistry: Stereochemistry,
    metadata: Option<AtomMetadata>,

    // computed properties
    bonds: Vec<Bond>,
    molecule_atom_index: Option<usize>,
}

#[allow(unused)]
impl Atom {
    fn new(
        atomic_number: usize,
        formal_charge: isize,
        is_aromatic: bool,
        name: Option<String>,
        molecule: Option<Molecule>,
        stereochemistry: Stereochemistry,
        metadata: Option<AtomMetadata>,
    ) -> Self {
        Self {
            atomic_number,
            formal_charge,
            is_aromatic,
            name,
            molecule,
            stereochemistry,
            metadata,
            bonds: Vec::new(),
            molecule_atom_index: None,
        }
    }

    fn add_bond(&mut self, bond: Bond) {
        self.bonds.push(bond);
    }

    fn metadata(&self) -> Option<&AtomMetadata> {
        self.metadata.as_ref()
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

    fn bonded_atoms(&self) -> Vec<Self> {
        let mut ret = Vec::new();
        for bond in &self.bonds {
            for atom in &bond.atoms {
                if atom != self {
                    ret.push(atom.clone());
                }
            }
        }
        ret
    }

    fn is_bonded_to(&self, other: &Self) -> bool {
        for bond in &self.bonds {
            for bonded_atom in &bond.atoms {
                if other == bonded_atom {
                    return true;
                }
            }
        }
        false
    }

    /// TODO potentially difficult. calls out to
    /// openff.toolkit.utils.toolkits.ToolkitRegistry to compute
    fn is_in_ring(&self) -> bool {
        todo!()
    }

    /// panics if `self` does not belong to a molecule
    fn molecule_atom_index(&mut self) -> Option<usize> {
        // TODO I don't like this panic but that's what the python does
        let Some(mol) = &self.molecule else {
            panic!("This Atom does not belong to a Molecule");
        };
        // this looks weird, but we need to compute it if it's not
        // already set
        if self.molecule_atom_index.is_some() {
            return self.molecule_atom_index;
        }
        let idx = mol.index(self);
        self.molecule_atom_index = idx;
        idx
    }
}

impl Particle for Atom {
    fn molecule(&self) -> Option<&Molecule> {
        self.molecule.as_ref()
    }

    fn set_molecule(&mut self, molecule: Molecule) {
        self.molecule = Some(molecule);
    }

    fn molecule_particle_index(&self) -> Option<usize> {
        if let Some(mol) = &self.molecule {
            mol.index(self)
        } else {
            None
        }
    }

    fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
}

#[derive(Clone, PartialEq)]
struct Bond {
    atom1: Atom,
    atom2: Atom,
    bond_order: usize,
    is_aromatic: bool,
    fractional_bond_order: Option<f64>,
    stereochemistry: Stereochemistry,
    atoms: Vec<Atom>,
}

impl Bond {
    #[allow(unused)]
    fn new(
        atom1: Atom,
        atom2: Atom,
        bond_order: usize,
        is_aromatic: bool,
        fractional_bond_order: Option<f64>,
        stereochemistry: Stereochemistry,
        atoms: Vec<Atom>,
    ) -> Self {
        let mut ret = Self {
            atom1,
            atom2,
            bond_order,
            is_aromatic,
            fractional_bond_order,
            stereochemistry,
            atoms,
        };
        ret.atom1.add_bond(ret.clone());
        ret.atom2.add_bond(ret.clone());
        ret
    }
}
