// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passleveltable {
    #[serde(rename = "basicRewardCount")]
    pub basic_reward_count: i32,
    #[serde(rename = "basicRewardId")]
    pub basic_reward_id: i32,
    #[serde(rename = "basicRewardType")]
    pub basic_reward_type: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "nextNeedExp")]
    pub next_need_exp: Option<i32>,
    #[serde(rename = "pass1RewardCount")]
    pub pass1_reward_count: Option<i32>,
    #[serde(rename = "pass1RewardId")]
    pub pass1_reward_id: Option<i32>,
    #[serde(rename = "pass1RewardType")]
    pub pass1_reward_type: Option<i32>,
    #[serde(rename = "premiumType")]
    pub premium_type: Option<i32>,
}

pub struct PassleveltableTable {
    records: Vec<Passleveltable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PassleveltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Passleveltable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Passleveltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Passleveltable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Passleveltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Passleveltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
