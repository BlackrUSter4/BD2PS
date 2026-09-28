// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charawaketable {
    #[serde(rename = "active")]
    pub active: i32,
    #[serde(rename = "growthId")]
    pub growth_id: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "imprintSlot1")]
    pub imprint_slot1: i32,
    #[serde(rename = "imprintSlot2")]
    pub imprint_slot2: i32,
    #[serde(rename = "imprintSlot3")]
    pub imprint_slot3: i32,
}

pub struct CharawaketableTable {
    records: Vec<Charawaketable>,
    by_id: HashMap<i32, usize>,
}

impl CharawaketableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Charawaketable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Charawaketable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Charawaketable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Charawaketable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
