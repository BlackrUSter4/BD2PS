// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeteriaeventtable {
    #[serde(rename = "TimelineName")]
    pub timeline_name: Option<String>,
    #[serde(rename = "bubbleIconSprite")]
    pub bubble_icon_sprite: String,
    #[serde(rename = "eventType")]
    pub event_type: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "reactionPrefabName")]
    pub reaction_prefab_name: Option<Vec<String>>,
    #[serde(rename = "reactionTime")]
    pub reaction_time: Option<Vec<i32>>,
    #[serde(rename = "rewardCount")]
    pub reward_count: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}

pub struct CafeteriaeventtableTable {
    records: Vec<Cafeteriaeventtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CafeteriaeventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeteriaeventtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cafeteriaeventtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Cafeteriaeventtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cafeteriaeventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Cafeteriaeventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
