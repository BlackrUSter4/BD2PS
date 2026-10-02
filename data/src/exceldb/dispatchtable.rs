// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispatchtable {
    #[serde(rename = "dispatchEndLocalTextId", default)]
    pub dispatch_end_local_text_id: i32,
    #[serde(rename = "dispatchLevel", default)]
    pub dispatch_level: i32,
    #[serde(rename = "dispatchLocalTextId", default)]
    pub dispatch_local_text_id: i32,
    #[serde(rename = "dispatchTime", default)]
    pub dispatch_time: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "nameTextId", default)]
    pub name_text_id: i32,
    #[serde(rename = "prefabName", default)]
    pub prefab_name: String,
    #[serde(rename = "rewardGroupId", default)]
    pub reward_group_id: i32,
    #[serde(rename = "spriteName", default)]
    pub sprite_name: String,
}

pub struct DispatchtableTable {
    records: Vec<Dispatchtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl DispatchtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Dispatchtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.reward_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Dispatchtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Dispatchtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Dispatchtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Dispatchtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
