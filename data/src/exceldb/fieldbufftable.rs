// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldbufftable {
    #[serde(rename = "buffDescLocalTextId", default)]
    pub buff_desc_local_text_id: i32,
    #[serde(rename = "buffLargeIcon", default)]
    pub buff_large_icon: Option<String>,
    #[serde(rename = "buffNameLocalTextId", default)]
    pub buff_name_local_text_id: i32,
    #[serde(rename = "buffSmallIcon", default)]
    pub buff_small_icon: Option<String>,
    #[serde(rename = "buffTime", default)]
    pub buff_time: Option<f32>,
    #[serde(rename = "buffType", default)]
    pub buff_type: Option<i32>,
    #[serde(rename = "disappearTime", default)]
    pub disappear_time: Option<f32>,
    #[serde(rename = "fieldDescLocalTextId", default)]
    pub field_desc_local_text_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "knockBack", default)]
    pub knock_back: Option<i32>,
    #[serde(rename = "renderType", default)]
    pub render_type: Option<i32>,
    #[serde(rename = "selectBuff", default)]
    pub select_buff: Option<i32>,
    #[serde(rename = "targetType", default)]
    pub target_type: Option<i32>,
    #[serde(rename = "value", default)]
    pub value: Option<f32>,
}

pub struct FieldbufftableTable {
    records: Vec<Fieldbufftable>,
    by_id: HashMap<i32, usize>,
}

impl FieldbufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldbufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldbufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldbufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldbufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
