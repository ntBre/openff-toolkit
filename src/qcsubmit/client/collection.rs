use std::{collections::HashMap, str::FromStr};

use crate::qcsubmit::results::TorsionDriveResultCollection;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize)]
struct QueryFilter {
    include: Option<bool>,
    exclude: Option<bool>,
}

#[derive(Clone, Serialize)]
struct Data {
    collection: String,
    name: String,
}

#[derive(Clone, Serialize)]
pub struct CollectionGetBody {
    meta: QueryFilter,
    data: Data,
}

#[derive(Clone, Copy)]
pub enum CollectionType {
    TorsionDrive,
    Optimization,
    SinglePoint,
}

impl From<CollectionType> for String {
    fn from(value: CollectionType) -> Self {
        match value {
            CollectionType::TorsionDrive => String::from("torsiondrivedataset"),
            CollectionType::Optimization => String::from("optimization"),
            CollectionType::SinglePoint => String::from("dataset"),
        }
    }
}

impl FromStr for CollectionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TorsionDrive" => Ok(Self::TorsionDrive),
            "Optimization" => Ok(Self::Optimization),
            "SinglePoint" => Ok(Self::SinglePoint),
            e => Err(format!("unmatched CollectionType: `{e}`")),
        }
    }
}

impl CollectionGetBody {
    /// Construct a new [CollectionGetBody] with `collection_type` and `name`.
    pub fn new(
        collection_type: CollectionType,
        name: impl Into<String>,
    ) -> Self {
        Self {
            meta: QueryFilter {
                include: None,
                exclude: None,
            },
            data: Data {
                collection: collection_type.into(),
                name: name.into(),
            },
        }
    }
}

#[derive(Debug, Default, Deserialize)]
struct Attributes {
    canonical_isomeric_explicit_hydrogen_mapped_smiles: String,
    inchi_key: String,
}

#[derive(Debug, Deserialize)]
pub struct TorsionDriveResult {
    pub name: String,

    attributes: Attributes,

    object_map: HashMap<String, String>,
}

impl TorsionDriveResult {
    /// return the record's id
    pub fn record_id(&self) -> &String {
        assert_eq!(self.object_map.len(), 1);
        self.object_map.get("default").unwrap()
    }

    #[inline]
    pub const fn cmiles(&self) -> &String {
        &self
            .attributes
            .canonical_isomeric_explicit_hydrogen_mapped_smiles
    }

    #[inline]
    pub const fn inchi_key(&self) -> &String {
        &self.attributes.inchi_key
    }
}

#[derive(Debug, Deserialize)]
pub struct BasicResult {
    pub molecule_id: String,
    pub name: String,
    pub comment: Option<String>,
    pub local_results: Value,
}

// I don't really like this. I would rather have a generic on Dataset, I think,
// but I was picturing the HashMap being generic over a result type.
// Unfortunately, the BasicResult isn't actually a HashMap at all, so this might
// just be the best solution other than a custom Deserialize implementation.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Records {
    Map(HashMap<String, TorsionDriveResult>),
    Vec(Vec<BasicResult>),
}

impl Records {
    pub fn into_values(self) -> Vec<TorsionDriveResult> {
        match self {
            Records::Map(m) => m.into_values().collect(),
            Records::Vec(v) => v
                .into_iter()
                .map(|r| TorsionDriveResult {
                    name: r.name,
                    attributes: Attributes::default(),
                    object_map: HashMap::new(),
                })
                .collect(),
        }
    }
}

/// the important fields in a [CollectionGetResponse]
#[derive(Debug, Deserialize)]
pub struct DataSet {
    pub id: String,
    pub collection: String,
    pub name: String,

    /// the keys are actually smiles strings, but they appear to be roughly the
    /// same as the `name` field on [Record] itself.
    pub records: Records,
}

#[derive(Debug, Deserialize)]
pub struct CollectionGetResponse {
    pub meta: HashMap<String, Value>,
    pub data: Vec<DataSet>,
}

impl CollectionGetResponse {
    pub fn ids(&self) -> Vec<String> {
        let mut ret = Vec::new();
        for ds in &self.data {
            match &ds.records {
                Records::Map(m) => {
                    ret.extend(m.values().map(|rec| rec.record_id().clone()))
                }
                Records::Vec(v) => {
                    ret.extend(v.iter().map(|rec| rec.molecule_id.clone()))
                }
            };
        }
        ret
    }
}

impl From<TorsionDriveResultCollection> for CollectionGetResponse {
    fn from(value: TorsionDriveResultCollection) -> Self {
        let mut records = HashMap::with_capacity(value.entries.len());
        for entries in value.entries.into_values() {
            for v in entries {
                records.insert(
                    v.record_id.clone(),
                    TorsionDriveResult {
                        name: v.cmiles.clone(),
                        attributes: Attributes {
                            canonical_isomeric_explicit_hydrogen_mapped_smiles: v.cmiles,
                            inchi_key: v.inchi_key },
                        object_map: HashMap::from([("default".to_string(), v.record_id)]),
                    },
                );
            }
        }
        Self {
            meta: HashMap::new(),
            data: vec![DataSet {
                id: String::new(),
                collection: String::new(),
                name: String::new(),
                records: Records::Map(records),
            }],
        }
    }
}
