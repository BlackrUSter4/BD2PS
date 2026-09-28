// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Passtable {
    #[serde(rename = "bannerFontLocalTextId")]
    pub banner_font_local_text_id: i32,
    #[serde(rename = "coreRewardCount")]
    pub core_reward_count: Option<i32>,
    #[serde(rename = "coreRewardId")]
    pub core_reward_id: Option<i32>,
    #[serde(rename = "coreRewardType")]
    pub core_reward_type: Option<i32>,
    #[serde(rename = "expEventMissionGroupId")]
    pub exp_event_mission_group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isShowCoreReward")]
    pub is_show_core_reward: Option<i32>,
    #[serde(rename = "mainBannerName")]
    pub main_banner_name: String,
    #[serde(rename = "newbiePassGroupId")]
    pub newbie_pass_group_id: Option<i32>,
    #[serde(rename = "newbiePassStep")]
    pub newbie_pass_step: Option<i32>,
    #[serde(rename = "passLevelGroupId")]
    pub pass_level_group_id: i32,
    #[serde(rename = "passNameTextId")]
    pub pass_name_text_id: i32,
    #[serde(rename = "passType")]
    pub pass_type: Option<i32>,
    #[serde(rename = "prefabName")]
    pub prefab_name: String,
    #[serde(rename = "scheduleType")]
    pub schedule_type: Option<i32>,
    #[serde(rename = "sortId")]
    pub sort_id: Option<i32>,
    #[serde(rename = "subBannerName")]
    pub sub_banner_name: String,
}

pub struct PasstableTable {
    records: Vec<Passtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PasstableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Passtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.exp_event_mission_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Passtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Passtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Passtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Passtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
