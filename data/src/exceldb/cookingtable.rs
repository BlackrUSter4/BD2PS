// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookingtable {
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "iconSpriteName", default)]
    pub icon_sprite_name: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "itemAcquireId", default)]
    pub item_acquire_id: Option<Vec<i32>>,
    #[serde(rename = "itemDescNameTextId", default)]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemSubNameTextId", default)]
    pub item_sub_name_text_id: i32,
    #[serde(rename = "materialItemCount", default)]
    pub material_item_count: Vec<i32>,
    #[serde(rename = "materialItemId", default)]
    pub material_item_id: Vec<i32>,
    #[serde(rename = "packId", default)]
    pub pack_id: i32,
    #[serde(rename = "recipeNameTextId", default)]
    pub recipe_name_text_id: i32,
    #[serde(rename = "resultItemCount", default)]
    pub result_item_count: i32,
    #[serde(rename = "resultItemId", default)]
    pub result_item_id: i32,
    #[serde(rename = "stackCount", default)]
    pub stack_count: i32,
    #[serde(rename = "talentLevel", default)]
    pub talent_level: i32,
}

pub struct CookingtableTable {
    records: Vec<Cookingtable>,
    by_id: HashMap<i32, usize>,
}

impl CookingtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cookingtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cookingtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cookingtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cookingtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
