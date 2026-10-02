// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pvpranktablev1 {
    #[serde(rename = "battleLoseRewardCount", default)]
    pub battle_lose_reward_count: Vec<i32>,
    #[serde(rename = "battleLoseRewardId", default)]
    pub battle_lose_reward_id: Vec<i32>,
    #[serde(rename = "battleLoseRewardType", default)]
    pub battle_lose_reward_type: Vec<i32>,
    #[serde(rename = "battleWinRewardCount", default)]
    pub battle_win_reward_count: Vec<i32>,
    #[serde(rename = "battleWinRewardId", default)]
    pub battle_win_reward_id: Vec<i32>,
    #[serde(rename = "battleWinRewardType", default)]
    pub battle_win_reward_type: Vec<i32>,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "losePoint", default)]
    pub lose_point: Option<i32>,
    #[serde(rename = "pvpRanking", default)]
    pub pvp_ranking: Option<i32>,
    #[serde(rename = "rankNameLocalTextId", default)]
    pub rank_name_local_text_id: i32,
    #[serde(rename = "seasonRewardCount", default)]
    pub season_reward_count: Vec<i32>,
    #[serde(rename = "seasonRewardId", default)]
    pub season_reward_id: Vec<i32>,
    #[serde(rename = "seasonRewardType", default)]
    pub season_reward_type: Vec<i32>,
    #[serde(rename = "seasonStartVP", default)]
    pub season_start_v_p: i32,
    #[serde(rename = "vp", default)]
    pub vp: i32,
    #[serde(rename = "winPoint", default)]
    pub win_point: i32,
}

pub struct Pvpranktablev1Table {
    records: Vec<Pvpranktablev1>,
    by_id: HashMap<i32, usize>,
}

impl Pvpranktablev1Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Pvpranktablev1> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Pvpranktablev1> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Pvpranktablev1] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Pvpranktablev1> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
