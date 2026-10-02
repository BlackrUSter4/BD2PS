// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fishingboattable {
    #[serde(rename = "boatBuffGroupId", default)]
    pub boat_buff_group_id: Vec<i32>,
    #[serde(rename = "costType", default)]
    pub cost_type: i32,
    #[serde(rename = "designId", default)]
    pub design_id: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
}

pub struct FishingboattableTable {
    records: Vec<Fishingboattable>,
    by_id: HashMap<i32, usize>,
}

impl FishingboattableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fishingboattable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fishingboattable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fishingboattable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fishingboattable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
