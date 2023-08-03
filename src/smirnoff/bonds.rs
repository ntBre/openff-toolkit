use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use super::Quantity;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bond {
    #[serde(rename = "@smirks")]
    pub smirks: String,

    #[serde(rename = "@k")]
    pub k: Quantity,

    #[serde(rename = "@id")]
    pub id: String,

    #[serde(rename = "@length")]
    pub length: Quantity,

    #[serde(rename = "@parameterize")]
    pub parameterize: Option<String>,
}

impl Bond {
    pub fn as_hash(&self, key: &str) -> Option<&Quantity> {
        match key {
            "length" => Some(&self.length),
            "k" => Some(&self.k),
            _ => None,
        }
    }

    pub fn as_hash_mut(&mut self, key: &str) -> Option<&mut Quantity> {
        match key {
            "length" => Some(&mut self.length),
            "k" => Some(&mut self.k),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bonds {
    #[serde(rename = "@version")]
    pub(crate) version: String,

    #[serde(rename = "@potential")]
    pub(crate) potential: String,

    #[serde(rename = "@fractional_bondorder_method")]
    pub(crate) fractional_bondorder_method: String,

    #[serde(rename = "@fractional_bondorder_interpolation")]
    pub(crate) fractional_bondorder_interpolation: String,

    #[serde(default, rename = "Bond")]
    pub bonds: Vec<Bond>,
}

impl Index<usize> for Bonds {
    type Output = Bond;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bonds[index]
    }
}

impl IndexMut<usize> for Bonds {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bonds[index]
    }
}

/// thanks to https://stackoverflow.com/a/73463595/12935407 for finally solving
/// this
impl<'a> IntoIterator for &'a Bonds {
    type Item = <&'a Vec<Bond> as IntoIterator>::Item;

    type IntoIter = <&'a Vec<Bond> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bonds.iter()
    }
}

impl IntoIterator for Bonds {
    type Item = Bond;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.bonds.into_iter()
    }
}
