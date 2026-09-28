// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questtable2004 {
    #[serde(rename = "acceptCinemaName")]
    pub accept_cinema_name: Option<String>,
    #[serde(rename = "charGroupId")]
    pub char_group_id: i32,
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
    #[serde(rename = "displayRewardCount1")]
    pub display_reward_count1: Vec<i32>,
    #[serde(rename = "displayRewardCount2")]
    pub display_reward_count2: Vec<i32>,
    #[serde(rename = "displayRewardId")]
    pub display_reward_id: Vec<i32>,
    #[serde(rename = "displayRewardId1")]
    pub display_reward_id1: Vec<i32>,
    #[serde(rename = "displayRewardId2")]
    pub display_reward_id2: Vec<i32>,
    #[serde(rename = "displayRewardType")]
    pub display_reward_type: Vec<i32>,
    #[serde(rename = "displayRewardType1")]
    pub display_reward_type1: Vec<i32>,
    #[serde(rename = "displayRewardType2")]
    pub display_reward_type2: Vec<i32>,
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
    #[serde(rename = "questCharIllustCostumeId")]
    pub quest_char_illust_costume_id: i32,
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
    #[serde(rename = "rewardCount1")]
    pub reward_count1: Vec<i32>,
    #[serde(rename = "rewardCount2")]
    pub reward_count2: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardId1")]
    pub reward_id1: Vec<i32>,
    #[serde(rename = "rewardId2")]
    pub reward_id2: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "rewardType1")]
    pub reward_type1: Vec<i32>,
    #[serde(rename = "rewardType2")]
    pub reward_type2: Vec<i32>,
    #[serde(rename = "timelineStartMapId")]
    pub timeline_start_map_id: i32,
}

pub struct Questtable2004Table {
    records: Vec<Questtable2004>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl Questtable2004Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questtable2004> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.char_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Questtable2004> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Questtable2004> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questtable2004] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Questtable2004> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
