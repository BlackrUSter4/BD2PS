// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamechartable {
    #[serde(rename = "amor", default)]
    pub amor: f32,
    #[serde(rename = "charGrowthId", default)]
    pub char_growth_id: i32,
    #[serde(rename = "charSkillGroupId", default)]
    pub char_skill_group_id: i32,
    #[serde(rename = "charUltimateSkillGroupId", default)]
    pub char_ultimate_skill_group_id: i32,
    #[serde(rename = "cooldownValue", default)]
    pub cooldown_value: f32,
    #[serde(rename = "costumeId", default)]
    pub costume_id: i32,
    #[serde(rename = "durationValue", default)]
    pub duration_value: f32,
    #[serde(rename = "expGainValue", default)]
    pub exp_gain_value: f32,
    #[serde(rename = "goldGainValue", default)]
    pub gold_gain_value: f32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "hpRecoveryPerSecond", default)]
    pub hp_recovery_per_second: f32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemPickupRange", default)]
    pub item_pickup_range: f32,
    #[serde(rename = "moveSpeed", default)]
    pub move_speed: f32,
    #[serde(rename = "powerValue", default)]
    pub power_value: f32,
    #[serde(rename = "prefabPath", default)]
    pub prefab_path: String,
    #[serde(rename = "projectileSpeed", default)]
    pub projectile_speed: f32,
    #[serde(rename = "rerollCount", default)]
    pub reroll_count: i32,
    #[serde(rename = "startHp", default)]
    pub start_hp: i32,
}

pub struct FieldminigamechartableTable {
    records: Vec<Fieldminigamechartable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamechartableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamechartable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.char_skill_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldminigamechartable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamechartable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamechartable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamechartable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
