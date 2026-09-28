// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastleranktable {
    #[serde(rename = "RewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "RewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "RewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rank")]
    pub rank: i32,
    #[serde(rename = "rankLocalTextId")]
    pub rank_local_text_id: i32,
}

pub struct EvilcastleranktableTable {
    records: Vec<Evilcastleranktable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastleranktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastleranktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastleranktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastleranktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Evilcastleranktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
