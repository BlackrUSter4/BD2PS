// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Foodtable {
    #[serde(rename = "favoriteRecoveryPoint")]
    pub favorite_recovery_point: i32,
    #[serde(rename = "foodType")]
    pub food_type: Option<i32>,
    #[serde(rename = "grade")]
    pub grade: i32,
    #[serde(rename = "iconSpriteName")]
    pub icon_sprite_name: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemAcquireId")]
    pub item_acquire_id: Option<Vec<i32>>,
    #[serde(rename = "itemDescNameTextId")]
    pub item_desc_name_text_id: i32,
    #[serde(rename = "itemNameTextId")]
    pub item_name_text_id: i32,
    #[serde(rename = "recoveryPoint")]
    pub recovery_point: i32,
    #[serde(rename = "sortType")]
    pub sort_type: i32,
    #[serde(rename = "stackCount")]
    pub stack_count: i32,
}

pub struct FoodtableTable {
    records: Vec<Foodtable>,
    by_id: HashMap<i32, usize>,
}

impl FoodtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Foodtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Foodtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Foodtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Foodtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
