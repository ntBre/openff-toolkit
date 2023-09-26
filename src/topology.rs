use ligand::molecule::Molecule;

pub mod molecule;

#[derive(Clone, Default)]
pub struct ChemicalEnvironment {
    pub topology_atom_indices: Vec<usize>,
}

#[derive(Clone)]
pub(crate) struct ChemicalEnvironmentMatch {
    #[allow(unused)]
    pub(crate) reference_molecule: Molecule,
    pub(crate) topology_atom_indices: Vec<usize>,
}

#[derive(Clone)]
pub struct Topology {
    pub molecules: Vec<Molecule>,
}

impl Topology {
    pub fn from_molecules(molecules: Vec<Molecule>) -> Self {
        Self { molecules }
    }

    #[cfg(feature = "openmm")]
    pub fn from_openmm(
        _topology: &ligand::molecule::Topology,
        _molecules: Vec<Molecule>,
    ) -> Self {
        todo!();
    }

    pub(crate) fn chemical_environment_matches(
        &self,
        _smirks: &str,
    ) -> Vec<ChemicalEnvironmentMatch> {
        todo!()
    }
}
