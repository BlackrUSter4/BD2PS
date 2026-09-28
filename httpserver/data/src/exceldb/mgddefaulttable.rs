// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mgddefaulttable {
    #[serde(rename = "dangerEffectTime")]
    pub danger_effect_time: i32,
    #[serde(rename = "dangerEffectValue")]
    pub danger_effect_value: i32,
    #[serde(rename = "elementAdvantage")]
    pub element_advantage: f32,
    #[serde(rename = "elementPenalty")]
    pub element_penalty: f32,
    #[serde(rename = "eventMissionGroupId")]
    pub event_mission_group_id: i32,
    #[serde(rename = "gameOverValue")]
    pub game_over_value: i32,
    #[serde(rename = "iconSpriteNameLarge")]
    pub icon_sprite_name_large: String,
    #[serde(rename = "iconSpriteNameSmall")]
    pub icon_sprite_name_small: String,
    #[serde(rename = "loadingLimit")]
    pub loading_limit: i32,
    #[serde(rename = "startCost")]
    pub start_cost: i32,
    #[serde(rename = "startWaitTime")]
    pub start_wait_time: i32,
    #[serde(rename = "summonCost")]
    pub summon_cost: i32,
}

pub struct MgddefaulttableTable {
    records: Vec<Mgddefaulttable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MgddefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mgddefaulttable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.event_mission_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Mgddefaulttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mgddefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Mgddefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
