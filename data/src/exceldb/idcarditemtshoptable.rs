// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idcarditemtshoptable {
    #[serde(rename = "buyMaxCount", default)]
    pub buy_max_count: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "priceCount", default)]
    pub price_count: Vec<i32>,
    #[serde(rename = "priceId", default)]
    pub price_id: Vec<i32>,
    #[serde(rename = "priceType", default)]
    pub price_type: Vec<i32>,
}

pub struct IdcarditemtshoptableTable {
    records: Vec<Idcarditemtshoptable>,
    by_id: HashMap<i32, usize>,
}

impl IdcarditemtshoptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Idcarditemtshoptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Idcarditemtshoptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Idcarditemtshoptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Idcarditemtshoptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
