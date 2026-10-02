// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myroomitemshoptable {
    #[serde(rename = "buyMaxCount", default)]
    pub buy_max_count: i32,
    #[serde(rename = "elementCount", default)]
    pub element_count: i32,
    #[serde(rename = "elementId", default)]
    pub element_id: i32,
    #[serde(rename = "elementType", default)]
    pub element_type: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "priceCount", default)]
    pub price_count: Vec<i32>,
    #[serde(rename = "priceId", default)]
    pub price_id: Vec<i32>,
    #[serde(rename = "priceType", default)]
    pub price_type: Vec<i32>,
}

pub struct MyroomitemshoptableTable {
    records: Vec<Myroomitemshoptable>,
    by_id: HashMap<i32, usize>,
}

impl MyroomitemshoptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Myroomitemshoptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Myroomitemshoptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Myroomitemshoptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Myroomitemshoptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
