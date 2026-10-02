// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlleveltable {
    #[serde(rename = "bossRoom", default)]
    pub boss_room: Vec<i32>,
    #[serde(rename = "enemyDamageRate", default)]
    pub enemy_damage_rate: f32,
    #[serde(rename = "enemyHealthRate", default)]
    pub enemy_health_rate: f32,
    #[serde(rename = "floorCount", default)]
    pub floor_count: i32,
    #[serde(rename = "getGoldRate", default)]
    pub get_gold_rate: f32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "nextSeasonOpenLevel", default)]
    pub next_season_open_level: i32,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Vec<i32>,
    #[serde(rename = "scoreBonusRate", default)]
    pub score_bonus_rate: Option<f32>,
    #[serde(rename = "spStartCount", default)]
    pub sp_start_count: Option<i32>,
    #[serde(rename = "spTurnAddCount", default)]
    pub sp_turn_add_count: i32,
}

pub struct RlleveltableTable {
    records: Vec<Rlleveltable>,
    by_id: HashMap<i32, usize>,
}

impl RlleveltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlleveltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlleveltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlleveltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlleveltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
