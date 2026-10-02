// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastletotalranktable {
    #[serde(rename = "RewardCount", default)]
    pub reward_count: Vec<i32>,
    #[serde(rename = "RewardId", default)]
    pub reward_id: Vec<i32>,
    #[serde(rename = "RewardType", default)]
    pub reward_type: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "rank", default)]
    pub rank: i32,
    #[serde(rename = "rankLocalTextId", default)]
    pub rank_local_text_id: i32,
}

pub struct EvilcastletotalranktableTable {
    records: Vec<Evilcastletotalranktable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastletotalranktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastletotalranktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastletotalranktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastletotalranktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Evilcastletotalranktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
