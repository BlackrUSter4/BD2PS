// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeterialeveltable {
    #[serde(rename = "bubbleCooldown")]
    pub bubble_cooldown: i32,
    #[serde(rename = "cafeteriaCostumeMax")]
    pub cafeteria_costume_max: i32,
    #[serde(rename = "cafeteriaCustomerNpcSpawnCount")]
    pub cafeteria_customer_npc_spawn_count: i32,
    #[serde(rename = "dayFxSoundResourceName")]
    pub day_fx_sound_resource_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapGrade")]
    pub map_grade: i32,
    #[serde(rename = "nightFxSoundResourceName")]
    pub night_fx_sound_resource_name: String,
    #[serde(rename = "skillType")]
    pub skill_type: Vec<i32>,
    #[serde(rename = "skillValue1")]
    pub skill_value1: Vec<i32>,
    #[serde(rename = "skillValue2")]
    pub skill_value2: Vec<i32>,
    #[serde(rename = "skillValue3")]
    pub skill_value3: Vec<i32>,
    #[serde(rename = "skillValue4")]
    pub skill_value4: Vec<i32>,
}

pub struct CafeterialeveltableTable {
    records: Vec<Cafeterialeveltable>,
    by_id: HashMap<i32, usize>,
}

impl CafeterialeveltableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeterialeveltable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cafeterialeveltable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cafeterialeveltable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cafeterialeveltable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
