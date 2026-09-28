// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobbysettingitemtable {
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: Vec<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "interactionId")]
    pub interaction_id: Option<i32>,
    #[serde(rename = "interactionResourceName")]
    pub interaction_resource_name: Option<String>,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: Option<i32>,
    #[serde(rename = "itemPath")]
    pub item_path: Option<Vec<String>>,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "prestigeSkinId")]
    pub prestige_skin_id: Option<i32>,
    #[serde(rename = "provideType")]
    pub provide_type: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "wallpaperBGFullPath")]
    pub wallpaper_b_g_full_path: Option<Vec<String>>,
    #[serde(rename = "wallpaperPath")]
    pub wallpaper_path: Option<Vec<String>>,
    #[serde(rename = "costumeId")]
    pub costume_id: Option<i32>,
    #[serde(rename = "wallPaperBGLoadType")]
    pub wall_paper_bg_load_type: Option<Vec<i32>>,
}

pub struct LobbysettingitemtableTable {
    records: Vec<Lobbysettingitemtable>,
    by_id: HashMap<i32, usize>,
}

impl LobbysettingitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Lobbysettingitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Lobbysettingitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Lobbysettingitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Lobbysettingitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
