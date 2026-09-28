// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventdroptable {
    #[serde(rename = "eventNameLocalTextId")]
    pub event_name_local_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "packId")]
    pub pack_id: Vec<i32>,
    #[serde(rename = "ratio")]
    pub ratio: i32,
    #[serde(rename = "rewardItemCount")]
    pub reward_item_count: i32,
    #[serde(rename = "rewardItemId")]
    pub reward_item_id: i32,
    #[serde(rename = "rewardItemType")]
    pub reward_item_type: i32,
}

pub struct EventdroptableTable {
    records: Vec<Eventdroptable>,
    by_id: HashMap<i32, usize>,
}

impl EventdroptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventdroptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventdroptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventdroptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Eventdroptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
