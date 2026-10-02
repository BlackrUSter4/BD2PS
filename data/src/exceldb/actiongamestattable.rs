// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamestattable {
    #[serde(rename = "criticalDamageRate", default)]
    pub critical_damage_rate: Option<f32>,
    #[serde(rename = "groggyHealthValue", default)]
    pub groggy_health_value: Option<i32>,
    #[serde(rename = "healthMaxValue", default)]
    pub health_max_value: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "moveSpeed", default)]
    pub move_speed: f32,
    #[serde(rename = "powerValue", default)]
    pub power_value: f32,
    #[serde(rename = "rageMaxValue", default)]
    pub rage_max_value: Option<i32>,
    #[serde(rename = "recoveryCount", default)]
    pub recovery_count: Option<i32>,
    #[serde(rename = "specialGaugeMaxValue", default)]
    pub special_gauge_max_value: Option<i32>,
    #[serde(rename = "staminaMaxValue", default)]
    pub stamina_max_value: Option<f32>,
    #[serde(rename = "staminaRecoveryValue", default)]
    pub stamina_recovery_value: Option<f32>,
}

pub struct ActiongamestattableTable {
    records: Vec<Actiongamestattable>,
    by_id: HashMap<i32, usize>,
}

impl ActiongamestattableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamestattable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamestattable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamestattable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actiongamestattable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
