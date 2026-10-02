// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Totalwarrewardtable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: i32,
    #[serde(rename = "rewardId", default)]
    pub reward_id: i32,
    #[serde(rename = "rewardType", default)]
    pub reward_type: i32,
    #[serde(rename = "score", default)]
    pub score: f32,
}

pub struct TotalwarrewardtableTable {
    records: Vec<Totalwarrewardtable>,
    by_id: HashMap<i32, usize>,
}

impl TotalwarrewardtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Totalwarrewardtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Totalwarrewardtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Totalwarrewardtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Totalwarrewardtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
