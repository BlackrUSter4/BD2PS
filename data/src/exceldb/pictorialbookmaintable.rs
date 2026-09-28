// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pictorialbookmaintable {
    #[serde(rename = "elementType")]
    pub element_type: Option<Vec<i32>>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "menuIcon")]
    pub menu_icon: Option<String>,
    #[serde(rename = "menuTitleLocalTextId")]
    pub menu_title_local_text_id: Option<i32>,
    #[serde(rename = "pictorialOrder")]
    pub pictorial_order: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
    #[serde(rename = "useBlind")]
    pub use_blind: Option<i32>,
}

pub struct PictorialbookmaintableTable {
    records: Vec<Pictorialbookmaintable>,
    by_id: HashMap<i32, usize>,
}

impl PictorialbookmaintableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Pictorialbookmaintable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Pictorialbookmaintable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Pictorialbookmaintable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Pictorialbookmaintable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
