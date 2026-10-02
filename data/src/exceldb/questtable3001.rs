// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questtable3001 {
    #[serde(rename = "acceptCinemaName", default)]
    pub accept_cinema_name: Option<String>,
    #[serde(rename = "completeCinemaName", default)]
    pub complete_cinema_name: String,
    #[serde(rename = "completeNpcId", default)]
    pub complete_npc_id: Option<i32>,
    #[serde(rename = "conditionCount", default)]
    pub condition_count: i32,
    #[serde(rename = "conditionType", default)]
    pub condition_type: Option<i32>,
    #[serde(rename = "displayMapId", default)]
    pub display_map_id: i32,
    #[serde(rename = "displayRewardCount", default)]
    pub display_reward_count: Vec<i32>,
    #[serde(rename = "displayRewardId", default)]
    pub display_reward_id: Vec<i32>,
    #[serde(rename = "displayRewardType", default)]
    pub display_reward_type: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "magicValue", default)]
    pub magic_value: Vec<i32>,
    #[serde(rename = "mapId", default)]
    pub map_id: i32,
    #[serde(rename = "nextQuestId", default)]
    pub next_quest_id: Option<i32>,
    #[serde(rename = "packId", default)]
    pub pack_id: i32,
    #[serde(rename = "priorQuestId", default)]
    pub prior_quest_id: Option<i32>,
    #[serde(rename = "prologSkipQuestTextId", default)]
    pub prolog_skip_quest_text_id: i32,
    #[serde(rename = "questConditionQuestTextId", default)]
    pub quest_condition_quest_text_id: i32,
    #[serde(rename = "questDescQuestTextId", default)]
    pub quest_desc_quest_text_id: i32,
    #[serde(rename = "questNameQuestTextId", default)]
    pub quest_name_quest_text_id: i32,
    #[serde(rename = "questSkipQuestTextId", default)]
    pub quest_skip_quest_text_id: i32,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType", default)]
    pub reward_type: Vec<i32>,
    #[serde(rename = "timelineStartMapId", default)]
    pub timeline_start_map_id: i32,
}

pub struct Questtable3001Table {
    records: Vec<Questtable3001>,
    by_id: HashMap<i32, usize>,
}

impl Questtable3001Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questtable3001> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Questtable3001> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questtable3001] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Questtable3001> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
