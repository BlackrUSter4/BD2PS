// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastleragetowertable {
    #[serde(rename = "battleDeckId")]
    pub battle_deck_id: i32,
    #[serde(rename = "battlePower")]
    pub battle_power: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isFixedCamera")]
    pub is_fixed_camera: Option<i32>,
}

pub struct EvilcastleragetowertableTable {
    records: Vec<Evilcastleragetowertable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastleragetowertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastleragetowertable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastleragetowertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastleragetowertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Evilcastleragetowertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
