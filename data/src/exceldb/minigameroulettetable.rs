// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minigameroulettetable {
    #[serde(rename = "freeCountDay", default)]
    pub free_count_day: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemCount", default)]
    pub item_count: i32,
    #[serde(rename = "itemId", default)]
    pub item_id: i32,
    #[serde(rename = "itemMaxConsume", default)]
    pub item_max_consume: i32,
    #[serde(rename = "itemType", default)]
    pub item_type: i32,
    #[serde(rename = "rouletteAccumulatedRewardGroupId", default)]
    pub roulette_accumulated_reward_group_id: i32,
    #[serde(rename = "rouletteRewardGroupId", default)]
    pub roulette_reward_group_id: i32,
    #[serde(rename = "rouletteUiPrefab", default)]
    pub roulette_ui_prefab: String,
}

pub struct MinigameroulettetableTable {
    records: Vec<Minigameroulettetable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MinigameroulettetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Minigameroulettetable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.roulette_accumulated_reward_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Minigameroulettetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Minigameroulettetable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Minigameroulettetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Minigameroulettetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
