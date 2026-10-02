// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clearevilcastletable {
    #[serde(rename = "contentsTicketId", default)]
    pub contents_ticket_id: i32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "rewardRandomBoxId", default)]
    pub reward_random_box_id: i32,
    #[serde(rename = "towerMagicId", default)]
    pub tower_magic_id: i32,
    #[serde(rename = "towerType", default)]
    pub tower_type: i32,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
}

pub struct ClearevilcastletableTable {
    records: Vec<Clearevilcastletable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ClearevilcastletableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Clearevilcastletable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Clearevilcastletable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Clearevilcastletable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Clearevilcastletable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
