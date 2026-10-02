// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Useitemtable {
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemDescNameTextId", default)]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId", default)]
    pub item_name_text_id: i32,
    #[serde(rename = "notTrash", default)]
    pub not_trash: i32,
    #[serde(rename = "sortType", default)]
    pub sort_type: i32,
    #[serde(rename = "stackCount", default)]
    pub stack_count: i32,
    #[serde(rename = "useItemType", default)]
    pub use_item_type: i32,
}

pub struct UseitemtableTable {
    records: Vec<Useitemtable>,
    by_id: HashMap<i32, usize>,
}

impl UseitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Useitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Useitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Useitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Useitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
