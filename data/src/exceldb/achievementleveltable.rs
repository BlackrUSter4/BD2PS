// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievementleveltable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "needEXP")]
    pub need_e_x_p: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
}

pub struct AchievementleveltableTable {
    records: Vec<Achievementleveltable>,
    by_id: HashMap<i32, usize>,
}

impl AchievementleveltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Achievementleveltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Achievementleveltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Achievementleveltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Achievementleveltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
