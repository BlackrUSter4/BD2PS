// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myroomtrophyitemtable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemSpriteName")]
    pub item_sprite_name: String,
    #[serde(rename = "objectDescNameTextId")]
    pub object_desc_name_text_id: i32,
    #[serde(rename = "objectNameTextId")]
    pub object_name_text_id: i32,
    #[serde(rename = "prefabName")]
    pub prefab_name: String,
}

pub struct MyroomtrophyitemtableTable {
    records: Vec<Myroomtrophyitemtable>,
    by_id: HashMap<i32, usize>,
}

impl MyroomtrophyitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Myroomtrophyitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Myroomtrophyitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Myroomtrophyitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Myroomtrophyitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
