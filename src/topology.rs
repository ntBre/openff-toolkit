use self::molecule::Molecule;

pub mod molecule;

pub struct ChemicalEnvironment {
    pub topology_atom_indices: Vec<usize>,
}

pub struct Topology {
    pub molecules: Vec<Molecule>,
}

impl Topology {
    pub fn from_molecules(molecules: Vec<Molecule>) -> Self {
        Self { molecules }
    }

    pub(crate) fn chemical_environment_matches(
        &self,
        smirks: &str,
    ) -> Vec<ChemicalEnvironment> {
        todo!()
    }
}
