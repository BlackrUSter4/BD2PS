// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collectiontable {
    #[serde(rename = "grade")]
    pub grade: i32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemDescNameTextId")]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: i32,
    #[serde(rename = "itemSubNameTextId")]
    pub item_sub_name_text_id: i32,
    #[serde(rename = "notTrash")]
    pub not_trash: i32,
    #[serde(rename = "itemAcquireNameTextId")]
    pub item_acquire_name_text_id: Option<i32>,
}

pub struct CollectiontableTable {
    records: Vec<Collectiontable>,
    by_id: HashMap<i32, usize>,
}

impl CollectiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Collectiontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Collectiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Collectiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Collectiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
