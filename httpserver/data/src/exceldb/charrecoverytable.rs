// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charrecoverytable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "recoveryAddItemCount")]
    pub recovery_add_item_count: i32,
    #[serde(rename = "recoveryItemCount")]
    pub recovery_item_count: i32,
    #[serde(rename = "recoveryItemType")]
    pub recovery_item_type: i32,
    #[serde(rename = "recoverySquadLevel")]
    pub recovery_squad_level: i32,
}

pub struct CharrecoverytableTable {
    records: Vec<Charrecoverytable>,
    by_id: HashMap<i32, usize>,
}

impl CharrecoverytableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Charrecoverytable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Charrecoverytable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Charrecoverytable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Charrecoverytable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
