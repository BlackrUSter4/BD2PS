// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packjameventtable {
    #[serde(rename = "insertMax")]
    pub insert_max: i32,
    #[serde(rename = "insertMin")]
    pub insert_min: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}

pub struct PackjameventtableTable {
    records: Vec<Packjameventtable>,
}

impl PackjameventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packjameventtable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Packjameventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packjameventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
