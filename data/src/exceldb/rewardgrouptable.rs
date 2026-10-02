// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rewardgrouptable {
    #[serde(rename = "dropCount", default)]
    pub drop_count: Option<i32>,
    #[serde(rename = "dropType", default)]
    pub drop_type: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemCount", default)]
    pub item_count: Option<Vec<i32>>,
    #[serde(rename = "itemId", default)]
    pub item_id: Option<Vec<i32>>,
    #[serde(rename = "itemType", default)]
    pub item_type: Option<Vec<i32>>,
    #[serde(rename = "mailId", default)]
    pub mail_id: Option<i32>,
    #[serde(rename = "ratio", default)]
    pub ratio: Option<Vec<i32>>,
}

pub struct RewardgrouptableTable {
    records: Vec<Rewardgrouptable>,
    by_id: HashMap<i32, usize>,
}

impl RewardgrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rewardgrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rewardgrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rewardgrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rewardgrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
