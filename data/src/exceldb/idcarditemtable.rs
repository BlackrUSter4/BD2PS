// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idcarditemtable {
    #[serde(rename = "filterId", default)]
    pub filter_id: i32,
    #[serde(rename = "filterNameTextId", default)]
    pub filter_name_text_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemDescNameTextId", default)]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId", default)]
    pub item_name_text_id: i32,
    #[serde(rename = "itemSpriteName", default)]
    pub item_sprite_name: String,
    #[serde(rename = "itemSubNameTextId", default)]
    pub item_sub_name_text_id: i32,
    #[serde(rename = "itemUnlockLocalTextId", default)]
    pub item_unlock_local_text_id: Option<i32>,
    #[serde(rename = "magicValue", default)]
    pub magic_value: Option<Vec<i32>>,
    #[serde(rename = "maxCount", default)]
    pub max_count: i32,
    #[serde(rename = "prefabName", default)]
    pub prefab_name: String,
    #[serde(rename = "sortId", default)]
    pub sort_id: Option<i32>,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
    #[serde(rename = "unlockConditionType", default)]
    pub unlock_condition_type: Option<i32>,
}

pub struct IdcarditemtableTable {
    records: Vec<Idcarditemtable>,
    by_id: HashMap<i32, usize>,
}

impl IdcarditemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Idcarditemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Idcarditemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Idcarditemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Idcarditemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
