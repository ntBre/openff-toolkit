//! [FractalClient] queries for procedures like [TorsionDrive]s and
//! [Optimization]s.

use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::qcsubmit::client::{Body, Status};

#[derive(Default, Serialize)]
struct QueryFilter {
    include: Option<usize>,
    exclude: Option<usize>,
    limit: Option<usize>,
    skip: usize,
}

#[derive(Serialize)]
struct Data {
    id: Vec<String>,
    task_id: Option<usize>,
    procedure: Option<usize>,
    program: Option<usize>,
    hash_index: Option<usize>,
    status: Status,
}

#[derive(Serialize)]
pub struct ProcedureGetBody {
    meta: QueryFilter,
    data: Data,
}

impl Body for ProcedureGetBody {
    fn new(id: Vec<String>) -> Self {
        Self {
            meta: QueryFilter::default(),
            data: Data {
                id,
                task_id: None,
                procedure: None,
                program: None,
                hash_index: None,
                status: Status::Complete,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptimizationSpec {
    pub program: String,
    pub keywords: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TDKeywords {
    dihedrals: Vec<(usize, usize, usize, usize)>,
    grid_spacing: Vec<isize>,
    dihedral_ranges: Option<Vec<(isize, isize)>>,
    energy_decrease_thresh: Option<f64>,
    energy_upper_limit: Option<f64>,
    #[serde(default)]
    additional_keywords: HashMap<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorsionDriveRecord {
    // base identification
    pub id: String,
    pub hash_index: Option<String>,
    pub procedure: String,
    pub program: String,
    pub version: usize,
    pub protocols: Option<HashMap<String, Value>>,
    pub extras: HashMap<String, Value>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub error: Option<String>,
    pub manager_name: Option<String>,
    pub status: Status,
    pub modified_on: String,
    pub created_on: String,
    pub provenance: Option<Value>,
    // input data
    pub initial_molecule: Vec<String>,
    pub keywords: TDKeywords,
    pub optimization_spec: OptimizationSpec,
    pub qc_spec: Value,
    // output data
    #[serde(rename = "final_energy_dict")]
    pub final_energies: HashMap<String, f64>,

    /// A map of grid points to additional ids
    /// Example:
    /// ```json
    /// "optimization_history": {
    ///   "[-120]": [
    ///     "104321688",
    ///     "104405676",
    ///     "104405677"
    ///   ]
    /// }
    /// ```
    pub optimization_history: HashMap<String, Vec<String>>,

    /// a map of something?
    ///
    /// ```json
    /// "minimum_positions": {
    ///   "[-15]": 0,
    ///   "[-30]": 0,
    ///   "[0]": 0,
    ///   "[-45]": 2,
    ///   "[15]": 1,
    ///   "[-60]": 1,
    /// }
    /// ```
    pub minimum_positions: HashMap<String, usize>,
}

impl TorsionDriveRecord {
    /// return an iterator over the optimization_id -> (record_id, grid_id)
    /// pairs in self.optimization_history. the keys are the ids of the
    /// OptimizationRecords associated with each point along the torsion drive
    pub(crate) fn optimizations(
        &self,
    ) -> impl Iterator<Item = (String, (String, String))> + '_ {
        self.minimum_positions.iter().map(|(grid_id, m)| {
            (
                self.optimization_history[grid_id][*m].clone(),
                (self.id.clone(), grid_id.clone()),
            )
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptimizationRecord {
    // base identification
    pub id: String,
    pub hash_index: Option<String>,
    pub procedure: String,
    pub program: String,
    pub version: usize,
    pub protocols: Option<HashMap<String, Value>>,
    pub extras: HashMap<String, Value>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub error: Option<String>,
    pub manager_name: Option<String>,
    pub status: Status,
    pub modified_on: String,
    pub created_on: String,
    pub provenance: Option<Value>,
    // input data
    pub initial_molecule: String,
    pub qc_spec: Value,
    pub keywords: HashMap<String, Value>,
    // results
    pub energies: Vec<f64>,
    pub final_molecule: String,
    pub trajectory: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub meta: Value,
    pub data: Vec<T>,
}

impl<T> IntoIterator for Response<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Response<TorsionDriveRecord> {
    pub fn optimization_ids(&self) -> Vec<String> {
        let mut ret = Vec::new();
        for record in &self.data {
            for (grid_id, minimum_idx) in &record.minimum_positions {
                ret.push(
                    record.optimization_history[grid_id][*minimum_idx].clone(),
                );
            }
        }
        ret
    }
}

impl Response<OptimizationRecord> {
    pub fn into_final_molecules(self) -> Vec<String> {
        self.data.into_iter().map(|r| r.final_molecule).collect()
    }

    pub fn final_molecules(&self) -> Vec<String> {
        self.data.iter().map(|r| r.final_molecule.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn check_opt_ids() {
        let s = read_to_string("testfiles/procedure.json").unwrap();
        let mut c: Response<TorsionDriveRecord> =
            serde_json::from_str(&s).unwrap();
        c.data.retain(|f| f.status.is_complete());
        let mut got_ids = c.optimization_ids();
        got_ids.sort();
        got_ids.dedup();
        let want_ids =
            read_to_string("testfiles/optimization_ids.txt").unwrap();
        let want_ids: Vec<&str> = want_ids.split_ascii_whitespace().collect();
        assert_eq!(got_ids, want_ids);
    }
}
