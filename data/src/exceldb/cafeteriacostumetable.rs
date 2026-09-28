// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeteriacostumetable {
    #[serde(rename = "costumeId")]
    pub costume_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
    #[serde(rename = "rewardValue")]
    pub reward_value: i32,
}

pub struct CafeteriacostumetableTable {
    records: Vec<Cafeteriacostumetable>,
    by_id: HashMap<i32, usize>,
}

impl CafeteriacostumetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeteriacostumetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cafeteriacostumetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cafeteriacostumetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cafeteriacostumetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
