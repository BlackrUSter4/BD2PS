// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monsterhuntrewardtable {
    #[serde(rename = "dailyRewardCount")]
    pub daily_reward_count: Vec<i32>,
    #[serde(rename = "dailyRewardId")]
    pub daily_reward_id: Vec<i32>,
    #[serde(rename = "dailyRewardType")]
    pub daily_reward_type: Vec<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
}

pub struct MonsterhuntrewardtableTable {
    records: Vec<Monsterhuntrewardtable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MonsterhuntrewardtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Monsterhuntrewardtable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Monsterhuntrewardtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Monsterhuntrewardtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Monsterhuntrewardtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
