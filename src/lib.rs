#![allow(unused)]

pub mod smirnoff;
pub mod topology;

// TODO this one goes in the openff-qcsubmit package
pub mod qcsubmit;

// TODO this is its own package
pub mod qcportal;

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        path::Path,
    };

    use crate::{
        qcportal::models::TorsionDriveRecord,
        qcsubmit::results::TorsionDriveResultCollection, smirnoff::ForceField,
        topology::molecule::Molecule,
    };

    use super::*;

    fn label_and_tag_ids(
        (record, molecule): (TorsionDriveRecord, Molecule),
        force_field: &ForceField,
        parameter_types: impl IntoIterator<Item = String>,
    ) -> HashSet<(String, String, usize)> {
        let mol_labels =
            &force_field.label_molecules(molecule.clone().to_topology())[0];

        let mut parameter_ids = HashSet::new();

        for parameter_type in parameter_types.into_iter() {
            let parameter_labels = mol_labels[&parameter_type].clone();
            let n_heavy_atoms = molecule
                .atoms
                .iter()
                .filter(|&a| a.atomic_number != 1)
                .count();

            for (indices, parameter) in parameter_labels {
                parameter_ids.insert((
                    parameter,
                    record.id.clone(),
                    n_heavy_atoms,
                ));
            }
        }

        parameter_ids
    }

    fn get_parameter_distribution(
        dataset: TorsionDriveResultCollection,
        force_field: &ForceField,
        parameter_types: impl IntoIterator<Item = String> + Clone,
    ) -> HashMap<String, usize> {
        let mut ret = HashMap::new();
        for record in dataset.to_records() {
            let parameter_ids =
                label_and_tag_ids(record, force_field, parameter_types.clone());
            for (parameter_id, record_id, n_heavy_atoms) in parameter_ids {
                *ret.entry(parameter_id).or_insert(0) += 1;
            }
        }
        ret
    }

    // TODO return some kind of Parameters type
    fn select_parameters<I>(
        dataset: TorsionDriveResultCollection,
        force_field: &ForceField,
        parameter_types: I,
    ) -> HashMap<String, Vec<String>>
    where
        I: IntoIterator<Item = String> + Clone,
    {
        // TODO this is an optional argument in the python version
        let min_coverage = 5;
        let coverage = get_parameter_distribution(
            dataset,
            force_field,
            parameter_types.clone(),
        );

        let mut ret = HashMap::new();
        for parameter_type in parameter_types.into_iter() {
            let handler = force_field.get_parameter_handler(&parameter_type);

            for (parameter_id, count) in &coverage {
                if *count < min_coverage {
                    continue;
                }
                let Some(parameter) = handler.get_parameter_by_id(parameter_id)
                else {
                    continue;
                };
                ret.entry(parameter_type.clone())
                    .or_insert(Vec::new())
                    .push(parameter.smirks().clone());
            }
        }
        ret
    }

    /// this is example code from my valence-fitting repo to combine two
    /// datasets. the python version took over 13 minutes to run
    #[test]
    fn combine() {
        let base = Path::new(
            "/home/brent/omsf/clone/sage-2.1.0/inputs-and-outputs/data-sets/",
        );
        let td = base.join("td-set-for-fitting-2.1.0.json");
        let sage_td = TorsionDriveResultCollection::parse_file(td).unwrap();
        let ff = ForceField::load(
            "/home/brent/omsf/projects/valence-fitting/01_generate-forcefield/output/initial-force-field-openff-2.1.0.offxml",
        )
        .unwrap();
        let selected_parameters =
            select_parameters(sage_td, &ff, ["ProperTorsions".to_owned()]);
    }
}
