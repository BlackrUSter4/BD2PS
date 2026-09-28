// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Characterquickuitable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "quickUi")]
    pub quick_ui: i32,
    #[serde(rename = "quickUiLocalTextId")]
    pub quick_ui_local_text_id: i32,
    #[serde(rename = "quickUiSpriteName")]
    pub quick_ui_sprite_name: String,
}

pub struct CharacterquickuitableTable {
    records: Vec<Characterquickuitable>,
    by_id: HashMap<i32, usize>,
}

impl CharacterquickuitableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Characterquickuitable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Characterquickuitable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Characterquickuitable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Characterquickuitable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
