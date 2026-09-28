// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worldbufftable {
    #[serde(rename = "buffIcon")]
    pub buff_icon: String,
    #[serde(rename = "buffNoticeLocalTextId")]
    pub buff_notice_local_text_id: i32,
    #[serde(rename = "buffTitleLocalTextId")]
    pub buff_title_local_text_id: i32,
    #[serde(rename = "buffValue")]
    pub buff_value: f32,
    #[serde(rename = "buffdescLocalTextId")]
    pub buffdesc_local_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "statType")]
    pub stat_type: i32,
}

pub struct WorldbufftableTable {
    records: Vec<Worldbufftable>,
    by_id: HashMap<i32, usize>,
}

impl WorldbufftableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Worldbufftable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Worldbufftable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Worldbufftable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Worldbufftable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
