// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookingresearchtable {
    #[serde(rename = "catalystValue")]
    pub catalyst_value: Vec<i32>,
    #[serde(rename = "failItemId")]
    pub fail_item_id: i32,
    #[serde(rename = "useSlotCount")]
    pub use_slot_count: Vec<i32>,
}

pub struct CookingresearchtableTable {
    records: Vec<Cookingresearchtable>,
}

impl CookingresearchtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cookingresearchtable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Cookingresearchtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Cookingresearchtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
