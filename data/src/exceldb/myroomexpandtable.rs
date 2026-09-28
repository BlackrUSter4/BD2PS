// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myroomexpandtable {
    #[serde(rename = "carpetLimitCount")]
    pub carpet_limit_count: i32,
    #[serde(rename = "charLimitCount")]
    pub char_limit_count: i32,
    #[serde(rename = "defaultRoom")]
    pub default_room: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "objectLimitCount")]
    pub object_limit_count: i32,
    #[serde(rename = "priceCount")]
    pub price_count: Option<i32>,
    #[serde(rename = "priceType")]
    pub price_type: Option<i32>,
}

pub struct MyroomexpandtableTable {
    records: Vec<Myroomexpandtable>,
    by_id: HashMap<i32, usize>,
}

impl MyroomexpandtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Myroomexpandtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Myroomexpandtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Myroomexpandtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Myroomexpandtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
