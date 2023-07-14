use std::{collections::HashMap, error::Error, fs::read_to_string};

use serde::Deserialize;

use crate::topology::{
    self, ChemicalEnvironment, ChemicalEnvironmentMatch, Topology,
};

#[derive(Clone, Debug, Deserialize)]
struct Constraint {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@distance")]
    distance: Option<String>, // TODO float + units ?
}

#[derive(Clone, Debug, Deserialize)]
struct Constraints {
    #[serde(rename = "@version")]
    version: String,

    #[serde(default, rename = "Constraint")]
    constraints: Vec<Constraint>,
}

#[derive(Clone, Debug, Deserialize)]
struct Bond {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@length")]
    length: String, // TODO float + units ?
}

#[derive(Clone, Debug, Deserialize)]
struct Bonds {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@potential")]
    potential: String,

    #[serde(rename = "@fractional_bondorder_method")]
    fractional_bondorder_method: String,

    #[serde(rename = "@fractional_bondorder_interpolation")]
    fractional_bondorder_interpolation: String,

    #[serde(default, rename = "Bond")]
    bonds: Vec<Bond>,
}

#[derive(Clone, Debug, Deserialize)]
struct Angle {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@angle")]
    angle: String, // TODO float + units ?
}

#[derive(Clone, Debug, Deserialize)]
struct Angles {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@potential")]
    potential: String,

    #[serde(default, rename = "Angle")]
    angles: Vec<Angle>,
}

#[derive(Clone, Debug, Deserialize)]
struct Proper {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    // TODO group these into a substruct for each of 1, 2, 3 if that's possible
    #[serde(rename = "@periodicity1")]
    periodicity1: String,

    #[serde(rename = "@phase1")]
    phase1: String,

    #[serde(rename = "@k1")]
    k1: String, // TODO float + units ?

    #[serde(rename = "@idivf1")]
    idivf1: String,

    #[serde(rename = "@periodicity2")]
    periodicity2: Option<String>,

    #[serde(rename = "@phase2")]
    phase2: Option<String>,

    #[serde(rename = "@k2")]
    k2: Option<String>, // TODO float + units ?

    #[serde(rename = "@idivf2")]
    idivf2: Option<String>,

    #[serde(rename = "@periodicity3")]
    periodicity3: Option<String>,

    #[serde(rename = "@phase3")]
    phase3: Option<String>,

    #[serde(rename = "@k3")]
    k3: Option<String>, // TODO float + units ?

    #[serde(rename = "@idivf3")]
    idivf3: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct ProperTorsions {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@potential")]
    potential: String,

    #[serde(rename = "@default_idivf")]
    default_idivf: String,

    #[serde(rename = "@fractional_bondorder_method")]
    fractional_bondorder_method: String,

    #[serde(rename = "@fractional_bondorder_interpolation")]
    fractional_bondorder_interpolation: String,

    #[serde(default, rename = "Proper")]
    proper_torsions: Vec<Proper>,
}

#[derive(Clone, Debug, Deserialize)]
struct Improper {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    // TODO group these into a substruct for each of 1, 2, 3 if that's possible
    #[serde(rename = "@periodicity1")]
    periodicity1: String,

    #[serde(rename = "@phase1")]
    phase1: String,

    #[serde(rename = "@k1")]
    k1: String, // TODO float + units ?
}

#[derive(Clone, Debug, Deserialize)]
struct ImproperTorsions {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@potential")]
    potential: String,

    #[serde(rename = "@default_idivf")]
    default_idivf: String,

    #[serde(default, rename = "Improper")]
    improper_torsions: Vec<Improper>,
}

#[derive(Clone, Debug, Deserialize)]
struct Atom {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@epsilon")]
    epsilon: String,

    #[serde(rename = "@rmin_half")]
    rmin_half: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct Vdw {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@potential")]
    potential: String,

    #[serde(rename = "@combining_rules")]
    combining_rules: String,

    #[serde(rename = "@scale12")]
    scale12: String,

    #[serde(rename = "@scale13")]
    scale13: String,

    #[serde(rename = "@scale14")]
    scale14: String,

    #[serde(rename = "@scale15")]
    scale15: String,

    #[serde(rename = "@cutoff")]
    cutoff: String,

    #[serde(rename = "@switch_width")]
    switch_width: String,

    #[serde(rename = "@method")]
    method: String,

    #[serde(default, rename = "Atom")]
    atoms: Vec<Atom>,
}

#[derive(Clone, Debug, Deserialize)]
struct Electrostatics {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@scale12")]
    scale12: String,

    #[serde(rename = "@scale13")]
    scale13: String,

    #[serde(rename = "@scale14")]
    scale14: String,

    #[serde(rename = "@scale15")]
    scale15: String,

    #[serde(rename = "@cutoff")]
    cutoff: String,

    #[serde(rename = "@switch_width")]
    switch_width: String,

    #[serde(rename = "@method")]
    method: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct LibraryCharge {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@charge1")]
    charge1: String,
}

#[derive(Clone, Debug, Deserialize)]
struct LibraryCharges {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "LibraryCharge")]
    library_charges: Vec<LibraryCharge>,
}

#[derive(Clone, Debug, Deserialize)]
struct ToolkitAM1BCC {
    #[serde(rename = "@version")]
    version: String,
}

/// A SMIRNOFF force field
#[derive(Clone, Debug, Deserialize)]
pub struct ForceField {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "@aromaticity_model")]
    aromaticity_model: String,

    #[serde(rename = "Author")]
    author: String,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Constraints")]
    constraints: Constraints,

    #[serde(rename = "Bonds")]
    bonds: Bonds,

    #[serde(rename = "Angles")]
    angles: Angles,

    #[serde(rename = "ProperTorsions")]
    proper_torsions: ProperTorsions,

    #[serde(rename = "ImproperTorsions")]
    improper_torsions: ImproperTorsions,

    #[serde(rename = "vdW")]
    vdw: Vdw,

    #[serde(rename = "Electrostatics")]
    electrostatics: Electrostatics,

    #[serde(rename = "LibraryCharges")]
    library_charges: LibraryCharges,

    #[serde(rename = "ToolkitAM1BCC")]
    toolkit_am1_bcc: ToolkitAM1BCC,
}

pub trait Parameter {
    fn id(&self) -> &String;
    fn smirks(&self) -> &String;
    fn typ(&self) -> &'static str;
}

/// implement the required getters for `Parameter`, assuming the corresponding
/// fields are present
macro_rules! impl_parameter {
    ($($type:ty $(,)*)*) => {
	$(
	    impl Parameter for $type {
		fn id(&self) -> &String {
		    &self.id
		}

		fn smirks(&self) -> &String {
		    &self.smirks
		}

		fn typ(&self) -> &'static str {
		    stringify!($type)
		}
	    }
	)*
    }
}

impl_parameter!(Bond, Angle, Proper, Improper);

struct Match {
    parameter_type: String,
    environment_match: ChemicalEnvironmentMatch,
}

impl Match {
    fn new(
        parameter_type: String,
        environment_match: ChemicalEnvironmentMatch,
    ) -> Self {
        Self {
            parameter_type,
            environment_match,
        }
    }
}

pub struct ParameterHandler {
    inner: Vec<Box<dyn Parameter>>,
}

impl ParameterHandler {
    #[allow(clippy::borrowed_box)]
    pub fn get_parameter_by_id(&self, id: &str) -> Option<&Box<dyn Parameter>> {
        self.inner.iter().find(|&p| p.id() == id)
    }

    fn find_matches(&self, entity: Topology) -> HashMap<Vec<usize>, Match> {
        let mut matches = HashMap::new();
        for parameter in &self.inner {
            let mut matches_for_this_type = HashMap::new();
            for environment_match in
                entity.chemical_environment_matches(parameter.smirks())
            {
                let handler_match = Match::new(
                    parameter.typ().to_owned(),
                    environment_match.clone(),
                );
                matches_for_this_type.insert(
                    environment_match.topology_atom_indices.clone(),
                    handler_match,
                );
            }

            matches.extend(matches_for_this_type);
        }

        matches
    }
}

impl ForceField {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(filename)?;
        let ff: Self = quick_xml::de::from_str(&contents)?;
        Ok(ff)
    }

    // TODO this should take an enum not string
    pub fn get_parameter_handler(
        &self,
        parameter_type: &str,
    ) -> ParameterHandler {
        let mut inner: Vec<Box<dyn Parameter>> = Vec::new();
        match parameter_type {
            "Bonds" => {
                for b in self.bonds.bonds.iter().cloned().map(Box::new) {
                    inner.push(b);
                }
            }
            "Angles" => {
                for b in self.angles.angles.iter().cloned().map(Box::new) {
                    inner.push(b);
                }
            }
            "ProperTorsions" => {
                for p in self
                    .proper_torsions
                    .proper_torsions
                    .iter()
                    .cloned()
                    .map(Box::new)
                {
                    inner.push(p);
                }
            }
            "ImproperTorsions" => {
                for b in self
                    .improper_torsions
                    .improper_torsions
                    .iter()
                    .cloned()
                    .map(Box::new)
                {
                    inner.push(b);
                }
            }
            _ => panic!("unrecognized parameter_type: {parameter_type}"),
        }
        ParameterHandler { inner }
    }

    fn parameter_handlers(&self) -> Vec<(&'static str, ParameterHandler)> {
        let mut ret = Vec::new();
        for typ in ["Bonds", "Angles", "ProperTorsions", "ImproperTorsions"] {
            ret.push((typ, self.get_parameter_handler(typ)))
        }
        ret
    }

    pub fn label_molecules(
        &self,
        topology: Topology,
    ) -> Vec<HashMap<String, HashMap<Vec<usize>, String>>> {
        let mut molecule_labels = Vec::new();

        for molecule in topology.molecules {
            let top_mol = Topology::from_molecules(vec![molecule]);
            let mut current_molecule_labels = HashMap::new();

            for (tag, parameter_handler) in self.parameter_handlers() {
                let matches = parameter_handler.find_matches(top_mol.clone());
                let mut parameter_matches = HashMap::new();
                for match_ in matches.keys() {
                    *parameter_matches
                        .entry(match_.clone())
                        .or_insert(String::new()) =
                        matches[match_].parameter_type.clone();
                }

                *current_molecule_labels.entry(tag.to_owned()).or_default() =
                    parameter_matches;
            }

            molecule_labels.push(current_molecule_labels);
        }

        molecule_labels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        let got = ForceField::load("testfiles/sage-2.1.0rc.offxml").unwrap();
    }
}
