// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipmentoptionrerolltable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lockItemCount")]
    pub lock_item_count: Vec<i32>,
    #[serde(rename = "rerollItemCount")]
    pub reroll_item_count: Vec<i32>,
    #[serde(rename = "rerollItemId")]
    pub reroll_item_id: Vec<i32>,
    #[serde(rename = "rerollItemType")]
    pub reroll_item_type: Vec<i32>,
}

pub struct EquipmentoptionrerolltableTable {
    records: Vec<Equipmentoptionrerolltable>,
    by_id: HashMap<i32, usize>,
}

impl EquipmentoptionrerolltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipmentoptionrerolltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Equipmentoptionrerolltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipmentoptionrerolltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Equipmentoptionrerolltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
