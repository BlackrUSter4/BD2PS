// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Randomboxtable {
    #[serde(rename = "dropType", default)]
    pub drop_type: Option<i32>,
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: Option<String>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemDescRandomBoxTextId", default)]
    pub item_desc_random_box_text_id: Option<i32>,
    #[serde(rename = "itemNameRandomBoxTextId", default)]
    pub item_name_random_box_text_id: Option<i32>,
    #[serde(rename = "notTrash", default)]
    pub not_trash: i32,
    #[serde(rename = "rewardGroupId", default)]
    pub reward_group_id: i32,
    #[serde(rename = "sortType", default)]
    pub sort_type: i32,
    #[serde(rename = "stackCount", default)]
    pub stack_count: Option<i32>,
}

pub struct RandomboxtableTable {
    records: Vec<Randomboxtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RandomboxtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Randomboxtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Randomboxtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Randomboxtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Randomboxtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Randomboxtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
