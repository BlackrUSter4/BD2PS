// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myroomitemtable {
    #[serde(rename = "canRotate")]
    pub can_rotate: Option<i32>,
    #[serde(rename = "charInteract")]
    pub char_interact: Option<Vec<i32>>,
    #[serde(rename = "dodgePrefabName")]
    pub dodge_prefab_name: Option<String>,
    #[serde(rename = "enumCountId")]
    pub enum_count_id: i32,
    #[serde(rename = "filterId")]
    pub filter_id: Option<i32>,
    #[serde(rename = "filterNameTextId")]
    pub filter_name_text_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemAnimation")]
    pub item_animation: Option<i32>,
    #[serde(rename = "itemInteract")]
    pub item_interact: Option<Vec<i32>>,
    #[serde(rename = "itemSpriteName")]
    pub item_sprite_name: Option<String>,
    #[serde(rename = "itemUnlockLocalTextId")]
    pub item_unlock_local_text_id: Option<i32>,
    #[serde(rename = "location")]
    pub location: Option<i32>,
    #[serde(rename = "maxCount")]
    pub max_count: i32,
    #[serde(rename = "objectDescNameTextId")]
    pub object_desc_name_text_id: Option<i32>,
    #[serde(rename = "objectNameTextId")]
    pub object_name_text_id: Option<i32>,
    #[serde(rename = "objectType")]
    pub object_type: Option<i32>,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "prefabName")]
    pub prefab_name: Option<String>,
    #[serde(rename = "questLevel")]
    pub quest_level: Option<i32>,
    #[serde(rename = "roomItemType")]
    pub room_item_type: Option<i32>,
}

pub struct MyroomitemtableTable {
    records: Vec<Myroomitemtable>,
    by_id: HashMap<i32, usize>,
}

impl MyroomitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Myroomitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Myroomitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Myroomitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Myroomitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
