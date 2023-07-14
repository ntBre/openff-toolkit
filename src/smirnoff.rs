#![allow(unused)]

use std::{error::Error, fs::read_to_string};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Constraint {
    #[serde(rename = "@smirks")]
    smirks: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@distance")]
    distance: String, // TODO float + units ?
}

#[derive(Debug, Deserialize)]
struct Constraints {
    #[serde(rename = "@version")]
    version: String,
    #[serde(default, rename = "Constraint")]
    constraints: Vec<Constraint>,
}

#[derive(Debug, Deserialize)]
struct Bond {
    #[serde(rename = "@smirks")]
    smirks: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@length")]
    length: String, // TODO float + units ?
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Angle {
    #[serde(rename = "@smirks")]
    smirks: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@angle")]
    angle: String, // TODO float + units ?
}

#[derive(Debug, Deserialize)]
struct Angles {
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "@potential")]
    potential: String,
    #[serde(default, rename = "Angle")]
    angles: Vec<Angle>,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
    method: String,
}

#[derive(Debug, Deserialize)]
struct LibraryCharge {
    #[serde(rename = "@smirks")]
    smirks: String,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@charge1")]
    charge1: String,
}

#[derive(Debug, Deserialize)]
struct LibraryCharges {
    #[serde(rename = "@version")]
    version: String,

    #[serde(rename = "LibraryCharge")]
    library_charges: Vec<LibraryCharge>,
}

#[derive(Debug, Deserialize)]
struct ToolkitAM1BCC {
    #[serde(rename = "@version")]
    version: String,
}

/// A SMIRNOFF force field
#[derive(Debug, Deserialize)]
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

impl ForceField {
    pub fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(filename)?;
        let ff: Self = quick_xml::de::from_str(&contents)?;
        Ok(ff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() {
        let got = ForceField::load("testfiles/sage-2.1.0rc.offxml").unwrap();
        dbg!(got);
    }
}
