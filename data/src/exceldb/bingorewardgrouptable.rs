// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bingorewardgrouptable {
    #[serde(rename = "clearCount")]
    pub clear_count: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: i32,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}

pub struct BingorewardgrouptableTable {
    records: Vec<Bingorewardgrouptable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl BingorewardgrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Bingorewardgrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Bingorewardgrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Bingorewardgrouptable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Bingorewardgrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Bingorewardgrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
