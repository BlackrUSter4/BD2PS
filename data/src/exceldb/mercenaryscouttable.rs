// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercenaryscouttable {
    #[serde(rename = "appearProb")]
    pub appear_prob: Option<i32>,
    #[serde(rename = "costumeId")]
    pub costume_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "presentItemCount")]
    pub present_item_count: Vec<i32>,
    #[serde(rename = "presentItemId")]
    pub present_item_id: Vec<i32>,
    #[serde(rename = "presentItemType")]
    pub present_item_type: Vec<i32>,
    #[serde(rename = "talkGroupId")]
    pub talk_group_id: i32,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct MercenaryscouttableTable {
    records: Vec<Mercenaryscouttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MercenaryscouttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mercenaryscouttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.talk_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Mercenaryscouttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Mercenaryscouttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mercenaryscouttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Mercenaryscouttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
