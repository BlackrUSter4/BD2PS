// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlbufftable {
    #[serde(rename = "conditionTiming")]
    pub condition_timing: Option<i32>,
    #[serde(rename = "conditionType")]
    pub condition_type: Option<i32>,
    #[serde(rename = "conditionValue")]
    pub condition_value: Option<i32>,
    #[serde(rename = "effectApplyType")]
    pub effect_apply_type: Option<i32>,
    #[serde(rename = "effectApplyValue")]
    pub effect_apply_value: Vec<i32>,
    #[serde(rename = "effectType")]
    pub effect_type: Option<i32>,
    #[serde(rename = "effectValue")]
    pub effect_value: f32,
    #[serde(rename = "id")]
    pub id: i32,
}

pub struct RlbufftableTable {
    records: Vec<Rlbufftable>,
    by_id: HashMap<i32, usize>,
}

impl RlbufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlbufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlbufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlbufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlbufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
