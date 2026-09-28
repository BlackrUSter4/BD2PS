// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventcoinexchangetable {
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "keyType")]
    pub key_type: Option<i32>,
    #[serde(rename = "limitedRatio")]
    pub limited_ratio: Option<i32>,
    #[serde(rename = "pageId")]
    pub page_id: i32,
    #[serde(rename = "ratio")]
    pub ratio: i32,
    #[serde(rename = "rewardItemCount")]
    pub reward_item_count: i32,
    #[serde(rename = "rewardItemId")]
    pub reward_item_id: Option<i32>,
    #[serde(rename = "rewardItemType")]
    pub reward_item_type: i32,
    #[serde(rename = "setCount")]
    pub set_count: i32,
}

pub struct EventcoinexchangetableTable {
    records: Vec<Eventcoinexchangetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EventcoinexchangetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventcoinexchangetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventcoinexchangetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Eventcoinexchangetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventcoinexchangetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Eventcoinexchangetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
