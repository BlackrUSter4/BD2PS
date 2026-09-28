// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventlostcointable {
    #[serde(rename = "eventPackId")]
    pub event_pack_id: Vec<i32>,
    #[serde(rename = "guideDescLocalTextId")]
    pub guide_desc_local_text_id: Vec<i32>,
    #[serde(rename = "guideTitleLocalTextId")]
    pub guide_title_local_text_id: Vec<i32>,
    #[serde(rename = "id")]
    pub id: i32,
}

pub struct EventlostcointableTable {
    records: Vec<Eventlostcointable>,
    by_id: HashMap<i32, usize>,
}

impl EventlostcointableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventlostcointable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventlostcointable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventlostcointable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Eventlostcointable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
