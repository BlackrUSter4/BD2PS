// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventstorytable {
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: Option<i32>,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "questConditionQuestTextId")]
    pub quest_condition_quest_text_id: i32,
    #[serde(rename = "questGroupId")]
    pub quest_group_id: i32,
    #[serde(rename = "questNameQuestTextId")]
    pub quest_name_quest_text_id: i32,
    #[serde(rename = "questTitleQuestTextId")]
    pub quest_title_quest_text_id: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "storyType")]
    pub story_type: Option<i32>,
    #[serde(rename = "timelineName")]
    pub timeline_name: String,
}

pub struct PackeventstorytableTable {
    records: Vec<Packeventstorytable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PackeventstorytableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventstorytable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Packeventstorytable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Packeventstorytable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventstorytable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packeventstorytable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
