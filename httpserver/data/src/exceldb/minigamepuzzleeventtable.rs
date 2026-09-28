// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Minigamepuzzleeventtable {
    #[serde(rename = "columnCount")]
    pub column_count: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemCount")]
    pub item_count: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "itemType")]
    pub item_type: i32,
    #[serde(rename = "puzzleCompleteRewardGroupId")]
    pub puzzle_complete_reward_group_id: i32,
    #[serde(rename = "puzzleRewardGroupId")]
    pub puzzle_reward_group_id: i32,
    #[serde(rename = "puzzleUiPrefab")]
    pub puzzle_ui_prefab: String,
    #[serde(rename = "tokenDescLocalTextId")]
    pub token_desc_local_text_id: i32,
    #[serde(rename = "tokenInfoLocation")]
    pub token_info_location: i32,
    #[serde(rename = "tokenShortCutId")]
    pub token_short_cut_id: i32,
    #[serde(rename = "tokenTitleLocalTextId")]
    pub token_title_local_text_id: i32,
}

pub struct MinigamepuzzleeventtableTable {
    records: Vec<Minigamepuzzleeventtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MinigamepuzzleeventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Minigamepuzzleeventtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.puzzle_complete_reward_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Minigamepuzzleeventtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Minigamepuzzleeventtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Minigamepuzzleeventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Minigamepuzzleeventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
