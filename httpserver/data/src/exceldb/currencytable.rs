// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currencytable {
    #[serde(rename = "iconSpriteNameLarge")]
    pub icon_sprite_name_large: Option<String>,
    #[serde(rename = "iconSpriteNameSmall")]
    pub icon_sprite_name_small: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "itemDescNameTextId")]
    pub item_desc_name_text_id: Option<i32>,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: Option<i32>,
    #[serde(rename = "targetItemId")]
    pub target_item_id: Option<i32>,
    #[serde(rename = "targetItemType")]
    pub target_item_type: Option<i32>,
}

pub struct CurrencytableTable {
    records: Vec<Currencytable>,
    by_id: HashMap<i32, usize>,
}

impl CurrencytableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Currencytable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            if let Some(id) = record.id {
                by_id.insert(id, idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Currencytable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Currencytable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Currencytable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
