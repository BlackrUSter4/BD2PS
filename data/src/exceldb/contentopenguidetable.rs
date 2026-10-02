// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contentopenguidetable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "magicValue", default)]
    pub magic_value: i32,
    #[serde(rename = "openDescLocalTextId", default)]
    pub open_desc_local_text_id: i32,
    #[serde(rename = "openIconName", default)]
    pub open_icon_name: String,
    #[serde(rename = "openTitleLocalTextId", default)]
    pub open_title_local_text_id: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

pub struct ContentopenguidetableTable {
    records: Vec<Contentopenguidetable>,
    by_id: HashMap<i32, usize>,
}

impl ContentopenguidetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Contentopenguidetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Contentopenguidetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Contentopenguidetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Contentopenguidetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
