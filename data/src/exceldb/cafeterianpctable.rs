// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeterianpctable {
    #[serde(rename = "gender")]
    pub gender: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "movePatternGroupId")]
    pub move_pattern_group_id: Option<i32>,
    #[serde(rename = "npcNameTextId")]
    pub npc_name_text_id: Option<i32>,
    #[serde(rename = "parttimePrefabName")]
    pub parttime_prefab_name: String,
    #[serde(rename = "parttimeType")]
    pub parttime_type: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<i32>,
}

pub struct CafeterianpctableTable {
    records: Vec<Cafeterianpctable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl CafeterianpctableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeterianpctable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            if let Some(group_id) = record.move_pattern_group_id {
                by_group.entry(group_id).or_insert_with(Vec::new).push(idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Cafeterianpctable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Cafeterianpctable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cafeterianpctable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cafeterianpctable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
