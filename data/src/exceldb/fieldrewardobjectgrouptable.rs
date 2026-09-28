// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldrewardobjectgrouptable {
    #[serde(rename = "buffId")]
    pub buff_id: Option<i32>,
    #[serde(rename = "castingSec")]
    pub casting_sec: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "miniMapViewFlag")]
    pub mini_map_view_flag: Option<i32>,
    #[serde(rename = "resetType")]
    pub reset_type: Option<i32>,
    #[serde(rename = "rewardGroupId")]
    pub reward_group_id: Option<i32>,
    #[serde(rename = "rewardIcon")]
    pub reward_icon: String,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "uiLocalTextId")]
    pub ui_local_text_id: i32,
}

pub struct FieldrewardobjectgrouptableTable {
    records: Vec<Fieldrewardobjectgrouptable>,
    by_id: HashMap<i32, usize>,
}

impl FieldrewardobjectgrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldrewardobjectgrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldrewardobjectgrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldrewardobjectgrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldrewardobjectgrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
