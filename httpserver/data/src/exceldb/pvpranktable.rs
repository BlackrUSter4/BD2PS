// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pvpranktable {
    #[serde(rename = "battleLoseRewardCount")]
    pub battle_lose_reward_count: Vec<i32>,
    #[serde(rename = "battleLoseRewardId")]
    pub battle_lose_reward_id: Vec<i32>,
    #[serde(rename = "battleLoseRewardType")]
    pub battle_lose_reward_type: Vec<i32>,
    #[serde(rename = "battleWinRewardCount")]
    pub battle_win_reward_count: Vec<i32>,
    #[serde(rename = "battleWinRewardId")]
    pub battle_win_reward_id: Vec<i32>,
    #[serde(rename = "battleWinRewardType")]
    pub battle_win_reward_type: Vec<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isELO")]
    pub is_e_l_o: Option<i32>,
    #[serde(rename = "losePoint")]
    pub lose_point: Option<i32>,
    #[serde(rename = "promotionRewardRandomBoxId")]
    pub promotion_reward_random_box_id: Option<i32>,
    #[serde(rename = "pvpRanking")]
    pub pvp_ranking: Option<i32>,
    #[serde(rename = "rankGroupId")]
    pub rank_group_id: i32,
    #[serde(rename = "rankGroupLocalTextId")]
    pub rank_group_local_text_id: i32,
    #[serde(rename = "rankNameLocalTextId")]
    pub rank_name_local_text_id: i32,
    #[serde(rename = "seasonRewardCount")]
    pub season_reward_count: Vec<i32>,
    #[serde(rename = "seasonRewardId")]
    pub season_reward_id: Vec<i32>,
    #[serde(rename = "seasonRewardType")]
    pub season_reward_type: Vec<i32>,
    #[serde(rename = "seasonStartVP")]
    pub season_start_v_p: i32,
    #[serde(rename = "vp")]
    pub vp: i32,
    #[serde(rename = "winPoint")]
    pub win_point: i32,
}

pub struct PvpranktableTable {
    records: Vec<Pvpranktable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PvpranktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Pvpranktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Pvpranktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Pvpranktable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Pvpranktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Pvpranktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
