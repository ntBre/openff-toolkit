use std::{collections::HashMap, marker::PhantomData};

use serde::{
    de::{IgnoredAny, Visitor},
    Deserialize, Serialize,
};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Filters {
    inner: Vec<Filter>,
}

struct FiltersVisitor;

impl<'de> Visitor<'de> for FiltersVisitor {
    type Value = Filters;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("a sequence of applied filters")
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = Filters {
            inner: Vec::with_capacity(access.size_hint().unwrap_or(0)),
        };

        while let Some(key) = access.next_key::<String>()? {
            let key =
                key.trim_end_matches(|c: char| c == '-' || c.is_ascii_digit());
            let filter = match key {
                "HydrogenBondFilter" => {
                    let mut map: HashMap<String, String> =
                        access.next_value()?;
                    let method = map.remove("method").ok_or_else(|| {
                        serde::de::Error::missing_field("method")
                    })?;
                    Filter::HydrogenBond { method }
                }
                "RecordStatusFilter" => {
                    let mut map: HashMap<String, String> =
                        access.next_value()?;
                    let status = map.remove("status").ok_or_else(|| {
                        serde::de::Error::missing_field("status")
                    })?;
                    Filter::RecordStatus { status }
                }
                "ConnectivityFilter" => {
                    let mut map: HashMap<String, f64> = access.next_value()?;
                    let tolerance =
                        map.remove("tolerance").ok_or_else(|| {
                            serde::de::Error::missing_field("tolerance")
                        })?;
                    Filter::Connectivity { tolerance }
                }
                "UnperceivableStereoFilter" => {
                    let mut map: HashMap<String, Vec<String>> =
                        access.next_value()?;
                    let toolkits = map.remove("toolkits").ok_or_else(|| {
                        serde::de::Error::missing_field("toolkits")
                    })?;
                    Filter::UnperceivableStereo { toolkits }
                }
                "ElementFilter" => {
                    let mut map: HashMap<String, Vec<String>> =
                        access.next_value()?;
                    let allowed_elements =
                        map.remove("allowed_elements").ok_or_else(|| {
                            serde::de::Error::missing_field("allowed_elements")
                        })?;
                    Filter::Element { allowed_elements }
                }
                _ => {
                    let _: IgnoredAny = access.next_value()?;
                    Filter::Misc {
                        name: key.to_owned(),
                    }
                }
            };
            map.inner.push(filter);
        }

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for Filters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FiltersVisitor)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Filter {
    HydrogenBond {
        method: String,
    },
    RecordStatus {
        status: String,
    },
    Connectivity {
        tolerance: f64,
    },
    UnperceivableStereo {
        toolkits: Vec<String>,
    },
    Element {
        allowed_elements: Vec<String>,
    },
    /// Catch-all for filters that we don't provide
    Misc {
        name: String,
    },
}
