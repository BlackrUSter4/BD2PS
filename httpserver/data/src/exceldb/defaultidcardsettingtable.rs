// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaultidcardsettingtable {
    #[serde(rename = "cardItemEnumType")]
    pub card_item_enum_type: i32,
    #[serde(rename = "colorValue")]
    pub color_value: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: Option<i32>,
    #[serde(rename = "positionXValue")]
    pub position_x_value: Option<f32>,
    #[serde(rename = "positionYValue")]
    pub position_y_value: Option<f32>,
    #[serde(rename = "scaleValue")]
    pub scale_value: f32,
}

pub struct DefaultidcardsettingtableTable {
    records: Vec<Defaultidcardsettingtable>,
    by_id: HashMap<i32, usize>,
}

impl DefaultidcardsettingtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Defaultidcardsettingtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Defaultidcardsettingtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Defaultidcardsettingtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Defaultidcardsettingtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
