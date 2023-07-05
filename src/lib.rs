pub mod topology {
    pub mod molecule {
        use std::collections::HashMap;

        #[derive(Clone, Copy, PartialEq)]
        enum Stereochemistry {
            R,
            S,
            None,
        }

        #[derive(PartialEq)]
        struct Molecule {
            atoms: Vec<Atom>,
        }

        impl Molecule {
            fn index(&self, atom: &Atom) -> Option<usize> {
                self.atoms.iter().position(|a| a == atom)
            }
        }

        type AtomMetadata = HashMap<String, String>;

        struct Quantity;

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
        #[derive(PartialEq)]
        struct Atom {
            atomic_number: usize,
            formal_charge: isize,
            is_aromatic: bool,
            name: Option<String>,
            molecule: Option<Molecule>,
            stereochemistry: Stereochemistry,
            metadata: Option<AtomMetadata>,
        }

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
                }
            }

            // fn add_bond(&mut self, bond: Bond);

            // fn metadata(&self) -> &AtomMetadata;

            // fn formal_charge(&self) -> isize;

            // fn set_formal_charge(&mut self, charge: isize);

            // fn partial_charge(&self) -> isize;

            // fn set_partial_charge(&mut self, charge: isize) -> isize;

            // fn is_aromatic(&self) -> bool;

            // fn stereochemistry(&self) -> Stereochemistry;

            // // NOTE below pasted in from python
            // fn set_stereochemistry(&mut self, stereochemistry: Stereochemistry);
            // fn atomic_number(&self) -> usize;
            // fn symbol(&self) -> &'static str;
            // fn mass(&self) -> Quantity;
            // fn name(&self);
            // fn set_name(&mut self, name: String);
            // fn bonds(&self);
            // fn bonded_atoms(&self) -> Vec<Self>;
            // fn is_bonded_to(self, other: &Self) -> bool;
            // fn is_in_ring(&self) -> bool;
            // fn molecule_atom_index(&self) -> usize;
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

        struct Bond {
            atom1: Atom,
            atom2: Atom,
        }
    }
}
