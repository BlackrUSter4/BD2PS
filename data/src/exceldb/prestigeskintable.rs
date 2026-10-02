// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prestigeskintable {
    #[serde(rename = "costumeId", default)]
    pub costume_id: i32,
    #[serde(rename = "effectTooltipDescLocalTextId", default)]
    pub effect_tooltip_desc_local_text_id: i32,
    #[serde(rename = "effectTooltipSpriteName", default)]
    pub effect_tooltip_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "lobbySettingItemId", default)]
    pub lobby_setting_item_id: i32,
    #[serde(rename = "skinNameTextId", default)]
    pub skin_name_text_id: i32,
}

pub struct PrestigeskintableTable {
    records: Vec<Prestigeskintable>,
    by_id: HashMap<i32, usize>,
}

impl PrestigeskintableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Prestigeskintable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Prestigeskintable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Prestigeskintable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Prestigeskintable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
