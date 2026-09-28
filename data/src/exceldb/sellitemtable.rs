// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sellitemtable {
    #[serde(rename = "discountRate")]
    pub discount_rate: i32,
    #[serde(rename = "elementCount")]
    pub element_count: i32,
    #[serde(rename = "elementId")]
    pub element_id: i32,
    #[serde(rename = "elementType")]
    pub element_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "premiumRate")]
    pub premium_rate: i32,
    #[serde(rename = "priceCount")]
    pub price_count: i32,
    #[serde(rename = "priceType")]
    pub price_type: i32,
}

pub struct SellitemtableTable {
    records: Vec<Sellitemtable>,
    by_id: HashMap<i32, usize>,
}

impl SellitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Sellitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Sellitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Sellitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Sellitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
