// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mgdupgradetable {
    #[serde(rename = "darkupgradeCost")]
    pub darkupgrade_cost: i32,
    #[serde(rename = "fireupgradeCost")]
    pub fireupgrade_cost: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lightupgradeCost")]
    pub lightupgrade_cost: i32,
    #[serde(rename = "waterupgradeCost")]
    pub waterupgrade_cost: i32,
    #[serde(rename = "windupgradeCost")]
    pub windupgrade_cost: i32,
}

pub struct MgdupgradetableTable {
    records: Vec<Mgdupgradetable>,
    by_id: HashMap<i32, usize>,
}

impl MgdupgradetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mgdupgradetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Mgdupgradetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mgdupgradetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Mgdupgradetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
