// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todayquesttable {
    #[serde(rename = "acceptDescQuestTextId")]
    pub accept_desc_quest_text_id: i32,
    #[serde(rename = "acceptNpcId")]
    pub accept_npc_id: i32,
    #[serde(rename = "completeNpcId")]
    pub complete_npc_id: Option<i32>,
    #[serde(rename = "conditionCount")]
    pub condition_count: i32,
    #[serde(rename = "conditionType")]
    pub condition_type: i32,
    #[serde(rename = "displayDifficulty")]
    pub display_difficulty: Option<i32>,
    #[serde(rename = "displayMapId")]
    pub display_map_id: i32,
    #[serde(rename = "displayRewardCount")]
    pub display_reward_count: Option<Vec<i32>>,
    #[serde(rename = "displayRewardId")]
    pub display_reward_id: Option<Vec<i32>>,
    #[serde(rename = "displayRewardType")]
    pub display_reward_type: Option<Vec<i32>>,
    #[serde(rename = "giveQuestItemId")]
    pub give_quest_item_id: Option<Vec<i32>>,
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
    #[serde(rename = "questConditionQuestTextId")]
    pub quest_condition_quest_text_id: i32,
    #[serde(rename = "questDescQuestTextId")]
    pub quest_desc_quest_text_id: i32,
    #[serde(rename = "questNameQuestTextId")]
    pub quest_name_quest_text_id: i32,
    #[serde(rename = "reputationCompleteId")]
    pub reputation_complete_id: Option<i32>,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<Vec<i32>>,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<Vec<i32>>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<Vec<i32>>,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct TodayquesttableTable {
    records: Vec<Todayquesttable>,
    by_id: HashMap<i32, usize>,
}

impl TodayquesttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Todayquesttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Todayquesttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Todayquesttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Todayquesttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
