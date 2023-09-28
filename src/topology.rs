use std::collections::{HashMap, HashSet};

use self::molecule::{Atom, Molecule};

pub mod molecule;

#[derive(Clone, Default)]
pub struct ChemicalEnvironment {
    pub topology_atom_indices: Vec<usize>,
}

#[derive(Clone)]
pub(crate) struct ChemicalEnvironmentMatch {
    pub(crate) reference_atom_indices: Vec<usize>,
    #[allow(unused)]
    pub(crate) reference_molecule: Molecule,
    pub(crate) topology_atom_indices: Vec<usize>,
}

impl ChemicalEnvironmentMatch {
    fn new(
        reference_atom_indices: Vec<usize>,
        reference_molecule: Molecule,
        topology_atom_indices: Vec<usize>,
    ) -> Self {
        Self {
            reference_atom_indices,
            reference_molecule,
            topology_atom_indices,
        }
    }
}

#[derive(Clone)]
pub struct Topology {
    pub molecules: Vec<Molecule>,
}

type IdenticalMolecules = HashMap<usize, Vec<(usize, HashMap<usize, usize>)>>;

impl Topology {
    pub fn from_molecules(molecules: Vec<Molecule>) -> Self {
        Self { molecules }
    }

    #[cfg(feature = "openmm")]
    pub fn from_openmm(
        _topology: &openmm::topology::Topology,
        molecules: Vec<Molecule>,
    ) -> Self {
        // my Topology doesn't actually do anything, so just skip all of this
        // work

        // use torx::Graph;

        // let mut omm_has_bond_orders = true;
        // for omm_bond in topology.bonds() {
        //     // NOTE we don't set bond orders anywhere in openmm so this is
        //     // always true:
        //     // if omm_bond.order.is_none() {
        //     if true {
        //         omm_has_bond_orders = false;
        //     }
        // }

        // let mut graphs = HashMap::new();
        // for mol in molecules {
        //     let graph = Graph::from(mol.clone());
        //     for existing_graph in graphs.keys() {
        //         if graph.is_isomorphic(existing_graph) {
        //             panic!("duplicate molecule error");
        //         }
        //     }
        //     graphs.insert(graph, mol);
        // }

        // // might have to define this as a function rather than a From impl
        // // depending on where Graph is defined
        // let top_graph = Graph::from(topology.clone());
        // let mut mols_to_add = Vec::new();
        // // this is supposed to be some connected_components and subgraph stuff,
        // // but we'll see if we can just iterate over nodes
        // for omm_mol_g in top_graph.nodes() {
        //     let mut match_found = false;
        // }

        Self { molecules }
    }

    pub(crate) fn chemical_environment_matches(
        &self,
        smirks: &str,
    ) -> Vec<ChemicalEnvironmentMatch> {
        let groupings = &self.identical_molecule_groups();

        let mut matches = Vec::new();
        for (unique_mol_idx, group) in groupings {
            let unique_mol = &self.molecules[*unique_mol_idx];
            let mol_matches = unique_mol.chemical_environment_matches(smirks);

            if mol_matches.is_empty() {
                continue;
            }

            for (mol_instance_idx, atom_map) in group {
                let mol_instance = &self.molecules[*mol_instance_idx];
                for mat in &mol_matches {
                    let mut topology_atom_indices = Vec::new();
                    for molecule_atom_index in &mat.reference_atom_indices {
                        let atom =
                            &mol_instance.atoms[atom_map[molecule_atom_index]];
                        topology_atom_indices.push(self.atom_index(atom));
                    }

                    matches.push(ChemicalEnvironmentMatch::new(
                        mat.reference_atom_indices.clone(),
                        unique_mol.clone(),
                        topology_atom_indices,
                    ));
                }
            }
        }

        matches
    }

    fn identify_chemically_identical_molecules(
        &self,
    ) -> HashMap<usize, (usize, HashMap<usize, usize>)> {
        let mut identity_maps = HashMap::new();
        let mut already_matched_mols = HashSet::new();

        for mol1_idx in 0..self.molecules.len() {
            if already_matched_mols.contains(&mol1_idx) {
                continue;
            }
            let mol1 = &self.molecules[mol1_idx];
            let mut map = HashMap::new();
            for i in 0..mol1.atoms.len() {
                map.insert(i, i);
            }
            identity_maps.insert(mol1_idx, (mol1_idx, map));

            for mol2_idx in mol1_idx + 1..self.molecules.len() {
                if already_matched_mols.contains(&mol2_idx) {
                    continue;
                }
                let mol2 = &self.molecules[mol2_idx];
                // wtf? why call on mol1 and pass mol1 ?? and why "are
                // isomorphic" instead of "is"
                let (are_isomorphic, atom_map) = mol1.are_isomorphic(mol2);

                // there's an isinstance check in python, but we'll assume the
                // types have to be the same for now. that will probably be
                // guaranteed by the type system in Rust if we do it right

                if are_isomorphic {
                    identity_maps.insert(mol2_idx, (mol1_idx, atom_map));
                    already_matched_mols.insert(mol2_idx);
                }
            }
        }

        identity_maps
    }

    fn identical_molecule_groups(&self) -> IdenticalMolecules {
        let identity_maps = self.identify_chemically_identical_molecules();
        let mut groupings = HashMap::new();
        for (molecule_idx, (unique_mol, atom_map)) in identity_maps {
            groupings
                .entry(unique_mol)
                .or_insert(Vec::new())
                .push((molecule_idx, atom_map));
        }
        groupings
    }

    fn atom_index(&self, _atom: &Atom) -> usize {
        todo!()
    }
}
