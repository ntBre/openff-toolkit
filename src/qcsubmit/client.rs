use std::{collections::HashMap, error::Error, fmt::Display};

use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use collection::{CollectionGetBody, CollectionGetResponse};
use molecule::{Molecule, MoleculeGetBody};
use procedure::{
    OptimizationRecord, ProcedureGetBody, Response, TorsionDriveRecord,
};

use self::collection::TorsionDriveResult;

mod collection;
mod molecule;
mod procedure;

#[cfg(test)]
mod tests {
    use crate::qcsubmit::{
        client::collection::CollectionType, results::ResultCollection,
    };

    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn de_response() {
        let s = read_to_string("testfiles/response.json").unwrap();
        let _c: CollectionGetResponse = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn de_singlept_response() {
        let s = read_to_string("testfiles/singlept_collection.json").unwrap();
        let _c: CollectionGetResponse = serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn de_procedure() {
        let s = read_to_string("testfiles/procedure.json").unwrap();
        let _c: Response<TorsionDriveRecord> =
            serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn de_opt_procedure() {
        let s = read_to_string("testfiles/opt_procedure.json").unwrap();
        let _c: Response<OptimizationRecord> =
            serde_json::from_str(&s).unwrap();
    }

    #[test]
    fn full() {
        let want = {
            let s = read_to_string("testfiles/final.dat").unwrap();
            let lines = s.lines();
            let mut ret = Vec::new();
            for line in lines {
                let sp: Vec<_> = line.split("=>").map(|s| s.trim()).collect();
                ret.push((
                    sp[0].to_owned(),
                    sp[1].to_owned(),
                    sp[2].parse::<usize>().unwrap(),
                ));
            }
            ret.sort_by_key(|r| r.0.clone());
            ret
        };

        let client = FractalClient::new();
        let col = CollectionGetBody::new(
            CollectionType::TorsionDrive,
            "OpenFF multiplicity correction torsion drive data v1.1",
        );

        let col = client.get_collection(col);
        let mut got = client.torsion_drive_records(col, 400);

        got.sort_by_key(|g| g.0.id.clone());
        let got: Vec<_> = got
            .into_iter()
            .map(|(a, b, c)| (a.id, b, c.len()))
            .collect();

        assert_eq!(got, want);
    }

    #[test]
    fn full_opt() {
        let want = {
            let s = read_to_string("testfiles/final_opt.dat").unwrap();
            let lines = s.lines();
            let mut ret = Vec::new();
            for line in lines {
                let sp: Vec<_> = line.split("=>").map(|s| s.trim()).collect();
                ret.push((
                    sp[0].to_owned(),
                    sp[1].to_owned(),
                    sp[2].parse::<usize>().unwrap(),
                ));
            }
            ret.sort_by_key(|r| r.0.clone());
            ret
        };

        let client = FractalClient::new();
        let ds =
            ResultCollection::parse_file("testfiles/core-opt.json").unwrap();
        let col: CollectionGetResponse = ds.into();
        let mut got = client.optimization_records(col, 400);

        got.sort_by_key(|g| g.0.id.clone());
        // NOTE: unlike above, comparing the length of the geometry (in atoms)
        // rather than the length of the conformers vector because it should
        // always contain a single conformer
        let got: Vec<_> = got
            .into_iter()
            .map(|(a, b, c)| (a.id, b, c[0].len() / 3))
            .collect();

        assert_eq!(got.len(), want.len());

        for (i, (g, w)) in got.iter().zip(want.iter()).enumerate() {
            assert_eq!(g, w, "mismatch at {i}: got:\n{g:#?}\nwant:\n{w:#?}\n");
        }
        assert_eq!(got, want);
    }
}

// TODO get rid of both make_*_results functions

/// constructs output usable by qcsubmit. Returns a vector of (record_id,
/// cmiles, Vec<geometry>), where a geometry is a Vec<f64> to be inserted in a
/// Molecule._conformers. There's not actually code in qcsubmit to do this
/// directly, but see results/caching.py:cached_query_torsion_drive_results for
/// how to reconstruct its output
pub fn make_td_results(
    results: Vec<TorsionDriveResult>,
    records: Vec<TorsionDriveRecord>,
    molecule_ids: HashMap<(String, String), String>,
    molecules: HashMap<String, Molecule>,
) -> Vec<(TorsionDriveRecord, String, Vec<Vec<f64>>)> {
    // there may be more results than records, but accessing them with this map
    // by the id stored on the records ensures that I only get the ones I want
    let cmiles_map: HashMap<_, _> = results
        .iter()
        .map(|rec| (rec.record_id(), rec.cmiles()))
        .collect();

    let mut ret = Vec::new();
    for record in records {
        let mut grid_ids: Vec<_> = record.minimum_positions.keys().collect();
        grid_ids.sort_by_key(|g| {
            let x: &[_] = &['[', ']'];
            g.trim_matches(x).parse::<isize>().unwrap()
        });

        let mut qc_grid_molecules = Vec::new();
        for grid_id in &grid_ids {
            let i = &molecule_ids[&(record.id.clone(), (*grid_id).clone())];
            qc_grid_molecules.push(molecules[i].clone());
        }

        let cmiles = cmiles_map[&record.id].clone();
        ret.push((
            record,
            cmiles,
            qc_grid_molecules.into_iter().map(|m| m.geometry).collect(),
        ));
    }

    ret
}

/// Analagous to [make_td_results] but without all of the bookkeeping mapping
/// individual molecules back to their corresponding TorsionDrives. Just pass in
/// a Vec<Molecule> and get back a Vec<(id, cmiles, Vec<Geometry>)>. The
/// Vec<Geometry> will always have length one. A vector is used just to keep the
/// return type consistent with the TorsionDrive version.
pub fn make_opt_results(
    results: Vec<TorsionDriveResult>,
    records: Vec<OptimizationRecord>,
    molecule_ids: HashMap<String, String>,
    molecules: HashMap<String, Molecule>,
) -> Vec<(OptimizationRecord, String, Vec<Vec<f64>>)> {
    // there may be more results than records, but accessing them with this map
    // by the id stored on the records ensures that I only get the ones I want
    let cmiles_map: HashMap<_, _> = results
        .iter()
        .map(|rec| (rec.record_id(), rec.cmiles()))
        .collect();

    let mut ret = Vec::new();
    for record in records {
        // do this first so we don't have to clone record.id
        let id = &molecule_ids[&record.id];
        // sad clones
        let geom = molecules[id].clone();
        let cmiles = cmiles_map[&record.id].clone();
        ret.push((record, cmiles, vec![geom.geometry]));
    }

    ret
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "ERROR")]
    Error,
}

impl Status {
    /// Returns `true` if the status is [`Complete`].
    ///
    /// [`Complete`]: Status::Complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

#[derive(Debug)]
struct ClientError;

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientError")
    }
}

impl Error for ClientError {}

pub trait ToJson {
    fn to_json(&self) -> Result<String, serde_json::Error>;
}

impl<T> ToJson for T
where
    T: Serialize,
{
    fn to_json(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Deserialize)]
pub struct Information {
    pub query_limit: usize,
}

impl Default for FractalClient {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Body {
    fn new(ids: Vec<String>) -> Self;
}

#[derive(Clone)]
pub struct FractalClient {
    address: &'static str,
    headers: HeaderMap,
    client: Client,
}

impl FractalClient {
    pub fn new() -> Self {
        const ADDR: &str = "https://api.qcarchive.molssi.org:443/";
        let mut ret = Self {
            address: ADDR,
            headers: HeaderMap::new(),
            client: Client::new(),
        };
        ret.headers
            .insert("Content-Type", "application/json".parse().unwrap());
        ret.headers
            .insert("User-Agent", "qcportal/0.15.7".parse().unwrap());
        ret
    }

    pub fn get_information(&self) -> Result<Information, Box<dyn Error>> {
        let url = format!("{}information", self.address);
        let response =
            self.client.get(url).headers(self.headers.clone()).send()?;
        if !response.status().is_success() {
            return Err(Box::new(ClientError));
        }
        let info: Information = response.json()?;
        Ok(info)
    }

    fn get(
        &self,
        endpoint: &str,
        body: impl ToJson,
    ) -> reqwest::blocking::Response {
        let url = format!("{}{endpoint}", self.address);
        let ret = self
            .client
            .get(url)
            .body(body.to_json().unwrap())
            .headers(self.headers.clone())
            .send()
            .unwrap();
        if !ret.status().is_success() {
            panic!("get `{endpoint}` failed with {ret:?}");
        }
        ret
    }

    pub fn get_collection(
        &self,
        body: CollectionGetBody,
    ) -> CollectionGetResponse {
        match self.get("collection", body.clone()).json() {
            Ok(r) => r,
            Err(_) => {
                // this is very stupid, but I'm tired of having to comment
                // things out and add todo!() to see the text
                std::fs::write(
                    "/tmp/out.json",
                    self.get("collection", body).text().unwrap(),
                )
                .unwrap();
                panic!("failed get_collection")
            }
        }
    }

    pub fn get_procedure<T: for<'a> Deserialize<'a>>(
        &self,
        body: ProcedureGetBody,
    ) -> Response<T> {
        self.get("procedure", body).json().unwrap()
    }

    pub fn get_molecule(&self, body: MoleculeGetBody) -> Response<Molecule> {
        self.get("molecule", body).json().unwrap()
    }

    /// Make an information request to the server to obtain the query limit
    pub fn get_query_limit(&self) -> usize {
        self.get_information().unwrap().query_limit
    }

    fn get_chunked<'a, B, R, Q>(
        &'a self,
        method: Q,
        ids: &[String],
        chunk_size: usize,
    ) -> Vec<R>
    where
        B: Body,
        Q: Fn(&'a FractalClient, B) -> R,
    {
        let mut futures = Vec::new();
        for chunk in ids.chunks(chunk_size) {
            let proc = B::new(chunk.to_vec());
            futures.push(method(self, proc));
        }
        futures
    }

    pub fn optimization_records(
        &self,
        collection: CollectionGetResponse,
        query_limit: usize,
    ) -> Vec<(OptimizationRecord, String, Vec<Vec<f64>>)> {
        // request the OptimizationRecords corresponding to the ids in the
        // collection
        let records: Vec<OptimizationRecord> = self
            .get_chunked(Self::get_procedure, &collection.ids(), query_limit)
            .into_iter()
            .flatten()
            .filter(|r: &OptimizationRecord| r.status.is_complete())
            .collect();

        eprintln!("{} optimization records", records.len());

        // get the molecule record ids corresponding to the final geometries.
        // molecule_ids is a map of final_molecule_ids -> original opt record
        // ids
        let mut molecule_ids = HashMap::with_capacity(records.len());
        for opt_record in &records {
            molecule_ids.insert(
                opt_record.id.clone(),
                opt_record.final_molecule.clone(),
            );
        }
        let ids: Vec<_> = molecule_ids.values().cloned().collect();

        eprintln!("asking for {} molecules", molecule_ids.len());

        // get the final molecules from each optimization trajectory and store
        // as a map of id -> mol
        let molecules: HashMap<_, _> = self
            .get_chunked(Self::get_molecule, &ids, query_limit)
            .into_iter()
            .flatten()
            .map(|m| (m.id.clone(), m))
            .collect();

        let results: Vec<_> = collection
            .data
            .into_iter()
            .flat_map(|ds| ds.records.into_values())
            .collect();

        make_opt_results(results, records, molecule_ids, molecules)
    }

    pub fn torsion_drive_records(
        &self,
        collection: CollectionGetResponse,
        query_limit: usize,
    ) -> Vec<(TorsionDriveRecord, String, Vec<Vec<f64>>)> {
        // request the TorsionDriveRecords corresponding to the ids in the
        // collection
        let records: Vec<TorsionDriveRecord> = self
            .get_chunked(Self::get_procedure, &collection.ids(), query_limit)
            .into_iter()
            .flatten()
            .filter(|r: &TorsionDriveRecord| r.status.is_complete())
            .collect();

        eprintln!("{} torsion drive records", records.len());

        // this is a map of optimization_id -> (record_id, grid_id)
        let mut intermediate_ids: HashMap<_, _> = records
            .iter()
            .flat_map(TorsionDriveRecord::optimizations)
            .collect();
        let optimization_ids: Vec<String> =
            intermediate_ids.keys().cloned().collect();

        // get the optimization records corresponding to each position in the
        // TorsionDrive
        let responses: Vec<OptimizationRecord> = self
            .get_chunked(Self::get_procedure, &optimization_ids, query_limit)
            .into_iter()
            .flatten()
            .collect();

        // this is a map of (record_id, grid_id) -> opt_record_id
        let mut molecule_ids = HashMap::with_capacity(optimization_ids.len());
        for opt_record in responses {
            molecule_ids.insert(
                intermediate_ids
                    .remove(&opt_record.id)
                    .expect("duplicate opt id?"),
                opt_record.final_molecule,
            );
        }
        let ids: Vec<_> = molecule_ids.values().cloned().collect();

        eprintln!("asking for {} molecules", ids.len());

        // get the final molecules from each optimization trajectory
        let molecules: HashMap<_, _> = self
            .get_chunked(Self::get_molecule, &ids, query_limit)
            .into_iter()
            .flatten()
            .map(|mol| (mol.id.clone(), mol))
            .collect();

        eprintln!("received {} molecules", molecules.len());

        let results: Vec<_> = collection
            .data
            .into_iter()
            .flat_map(|ds| ds.records.into_values())
            .collect();

        make_td_results(results, records, molecule_ids, molecules)
    }
}
