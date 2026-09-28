// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questtable3005 {
    #[serde(rename = "acceptCinemaName")]
    pub accept_cinema_name: Option<String>,
    #[serde(rename = "collectionId")]
    pub collection_id: Option<Vec<i32>>,
    #[serde(rename = "completeCinemaName")]
    pub complete_cinema_name: String,
    #[serde(rename = "conditionCount")]
    pub condition_count: i32,
    #[serde(rename = "conditionType")]
    pub condition_type: i32,
    #[serde(rename = "displayMapId")]
    pub display_map_id: i32,
    #[serde(rename = "displayRewardCount")]
    pub display_reward_count: Vec<i32>,
    #[serde(rename = "displayRewardId")]
    pub display_reward_id: Vec<i32>,
    #[serde(rename = "displayRewardType")]
    pub display_reward_type: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "magicValue")]
    pub magic_value: Vec<i32>,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "nextQuestId")]
    pub next_quest_id: Option<i32>,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "priorQuestId")]
    pub prior_quest_id: Option<i32>,
    #[serde(rename = "prologSkipQuestTextId")]
    pub prolog_skip_quest_text_id: i32,
    #[serde(rename = "questConditionQuestTextId")]
    pub quest_condition_quest_text_id: i32,
    #[serde(rename = "questDescQuestTextId")]
    pub quest_desc_quest_text_id: i32,
    #[serde(rename = "questNameQuestTextId")]
    pub quest_name_quest_text_id: i32,
    #[serde(rename = "questSkipQuestTextId")]
    pub quest_skip_quest_text_id: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "timelineStartMapId")]
    pub timeline_start_map_id: i32,
}

pub struct Questtable3005Table {
    records: Vec<Questtable3005>,
    by_id: HashMap<i32, usize>,
}

impl Questtable3005Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questtable3005> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Questtable3005> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questtable3005] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Questtable3005> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
