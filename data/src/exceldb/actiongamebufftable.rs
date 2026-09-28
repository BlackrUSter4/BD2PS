// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamebufftable {
    #[serde(rename = "buffActiveType")]
    pub buff_active_type: i32,
    #[serde(rename = "buffApplyType")]
    pub buff_apply_type: i32,
    #[serde(rename = "buffDescNameTextId")]
    pub buff_desc_name_text_id: Option<i32>,
    #[serde(rename = "buffEffectName")]
    pub buff_effect_name: Option<String>,
    #[serde(rename = "buffNameTextId")]
    pub buff_name_text_id: Option<i32>,
    #[serde(rename = "buffSpriteName")]
    pub buff_sprite_name: Option<String>,
    #[serde(rename = "buffTime")]
    pub buff_time: f32,
    #[serde(rename = "buffValue")]
    pub buff_value: Option<f32>,
    #[serde(rename = "classType")]
    pub class_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "subMagicValue")]
    pub sub_magic_value: Option<Vec<f32>>,
}

pub struct ActiongamebufftableTable {
    records: Vec<Actiongamebufftable>,
    by_id: HashMap<i32, usize>,
}

impl ActiongamebufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamebufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamebufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamebufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actiongamebufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
