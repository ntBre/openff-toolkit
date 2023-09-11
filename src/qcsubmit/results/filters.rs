use std::collections::HashMap;

use serde::{
    de::{IgnoredAny, Visitor},
    ser::SerializeMap,
    Deserialize, Serialize, Serializer,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Filters(pub Vec<Filter>);

impl Filters {
    fn new() -> Self {
        Self(Vec::new())
    }
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
        let mut map = Filters::new();

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
                    Filter::Misc(key.to_owned())
                }
            };
            map.0.push(filter);
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

impl Serialize for Filters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for (count, e) in self.0.iter().enumerate() {
            match e {
                Filter::HydrogenBond { method } => map.serialize_entry(
                    &format!("HydrogenBondFilter-{count}"),
                    &HashMap::from([("method", method)]),
                )?,
                Filter::RecordStatus { status } => map.serialize_entry(
                    &format!("RecordStatusFilter-{count}"),
                    &HashMap::from([("status", status)]),
                )?,
                Filter::Connectivity { tolerance } => map.serialize_entry(
                    &format!("ConnectivityFilter-{count}"),
                    &HashMap::from([("tolerance", tolerance)]),
                )?,
                Filter::UnperceivableStereo { toolkits } => map
                    .serialize_entry(
                        &format!("UnperceivableStereoFilter-{count}"),
                        &HashMap::from([("toolkits", toolkits)]),
                    )?,
                Filter::Element { allowed_elements } => map.serialize_entry(
                    &format!("ElementFilter-{count}"),
                    &HashMap::from([("allowed_elements", allowed_elements)]),
                )?,
                Filter::Misc(name) => map
                    .serialize_entry::<_, HashMap<(), ()>>(
                        &format!("{name}-{count}"),
                        &HashMap::from([]),
                    )?,
            }
        }
        map.end()
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
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
    Misc(String),
}
