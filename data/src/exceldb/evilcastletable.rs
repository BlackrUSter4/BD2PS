// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastletable {
    #[serde(rename = "PointPositionId", default)]
    pub point_position_id: i32,
    #[serde(rename = "RewardCount", default)]
    pub reward_count: Vec<i32>,
    #[serde(rename = "RewardId", default)]
    pub reward_id: Vec<i32>,
    #[serde(rename = "RewardType", default)]
    pub reward_type: Vec<i32>,
    #[serde(rename = "battlePower", default)]
    pub battle_power: i32,
    #[serde(rename = "bestRecordTimeline", default)]
    pub best_record_timeline: String,
    #[serde(rename = "bossDescLocalTextId", default)]
    pub boss_desc_local_text_id: i32,
    #[serde(rename = "bossId", default)]
    pub boss_id: i32,
    #[serde(rename = "bossNameTextId", default)]
    pub boss_name_text_id: i32,
    #[serde(rename = "bossTargetLocalText", default)]
    pub boss_target_local_text: i32,
    #[serde(rename = "clearDescLocalTextId", default)]
    pub clear_desc_local_text_id: i32,
    #[serde(rename = "floorDescLocalTextId", default)]
    pub floor_desc_local_text_id: i32,
    #[serde(rename = "floorNameLocalTextId", default)]
    pub floor_name_local_text_id: i32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "mapId", default)]
    pub map_id: i32,
    #[serde(rename = "monsterId", default)]
    pub monster_id: Vec<i32>,
    #[serde(rename = "normalDescLocalTextId", default)]
    pub normal_desc_local_text_id: i32,
    #[serde(rename = "normalTargetLocalText", default)]
    pub normal_target_local_text: i32,
    #[serde(rename = "recordUpdateTimeline", default)]
    pub record_update_timeline: String,
    #[serde(rename = "timeAttackId", default)]
    pub time_attack_id: i32,
}

pub struct EvilcastletableTable {
    records: Vec<Evilcastletable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastletableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastletable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastletable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastletable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Evilcastletable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
