// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamedefaulttable {
    #[serde(rename = "aggroDecreaseValue")]
    pub aggro_decrease_value: f32,
    #[serde(rename = "alertStateBuffId")]
    pub alert_state_buff_id: i32,
    #[serde(rename = "blowDamageSoundName")]
    pub blow_damage_sound_name: String,
    #[serde(rename = "brokenBuffId")]
    pub broken_buff_id: i32,
    #[serde(rename = "cutDamageSoundName")]
    pub cut_damage_sound_name: String,
    #[serde(rename = "damageLimit")]
    pub damage_limit: i32,
    #[serde(rename = "emergencyHpRate")]
    pub emergency_hp_rate: f32,
    #[serde(rename = "eventMissionGroupId")]
    pub event_mission_group_id: i32,
    #[serde(rename = "healEffectPrefabName")]
    pub heal_effect_prefab_name: String,
    #[serde(rename = "justDodgeEffectPrefabName")]
    pub just_dodge_effect_prefab_name: String,
    #[serde(rename = "justDodgeInvincibleTime")]
    pub just_dodge_invincible_time: f32,
    #[serde(rename = "loadingLimit")]
    pub loading_limit: i32,
    #[serde(rename = "outGameBgmName")]
    pub out_game_bgm_name: String,
    #[serde(rename = "rageDamageValue")]
    pub rage_damage_value: f32,
    #[serde(rename = "rankMaxCount")]
    pub rank_max_count: i32,
    #[serde(rename = "skillHoldThreshold")]
    pub skill_hold_threshold: f32,
}

pub struct ActiongamedefaulttableTable {
    records: Vec<Actiongamedefaulttable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ActiongamedefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamedefaulttable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.event_mission_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Actiongamedefaulttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamedefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Actiongamedefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
