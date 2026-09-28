// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastletable {
    #[serde(rename = "PointPositionId")]
    pub point_position_id: i32,
    #[serde(rename = "RewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "RewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "RewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "battlePower")]
    pub battle_power: i32,
    #[serde(rename = "bestRecordTimeline")]
    pub best_record_timeline: String,
    #[serde(rename = "bossDescLocalTextId")]
    pub boss_desc_local_text_id: i32,
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "bossNameTextId")]
    pub boss_name_text_id: i32,
    #[serde(rename = "bossTargetLocalText")]
    pub boss_target_local_text: i32,
    #[serde(rename = "clearDescLocalTextId")]
    pub clear_desc_local_text_id: i32,
    #[serde(rename = "floorDescLocalTextId")]
    pub floor_desc_local_text_id: i32,
    #[serde(rename = "floorNameLocalTextId")]
    pub floor_name_local_text_id: i32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "monsterId")]
    pub monster_id: Vec<i32>,
    #[serde(rename = "normalDescLocalTextId")]
    pub normal_desc_local_text_id: i32,
    #[serde(rename = "normalTargetLocalText")]
    pub normal_target_local_text: i32,
    #[serde(rename = "recordUpdateTimeline")]
    pub record_update_timeline: String,
    #[serde(rename = "timeAttackId")]
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
    pub fn iter(&self) -> std::slice::Iter<Evilcastletable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
