// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questitemtable {
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemDescNameTextId")]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: i32,
    #[serde(rename = "notTrash")]
    pub not_trash: i32,
    #[serde(rename = "stackCount")]
    pub stack_count: i32,
    #[serde(rename = "itemAcquireNameTextId")]
    pub item_acquire_name_text_id: Option<i32>,
}

pub struct QuestitemtableTable {
    records: Vec<Questitemtable>,
    by_id: HashMap<i32, usize>,
}

impl QuestitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Questitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Questitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
