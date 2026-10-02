// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Titleitemtable {
    #[serde(rename = "grade", default)]
    pub grade: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "titleItemName", default)]
    pub title_item_name: String,
    #[serde(rename = "titleMissionTextId", default)]
    pub title_mission_text_id: i32,
    #[serde(rename = "titleNameTextId", default)]
    pub title_name_text_id: String,
    #[serde(rename = "titleSortNumber", default)]
    pub title_sort_number: i32,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
}

pub struct TitleitemtableTable {
    records: Vec<Titleitemtable>,
    by_id: HashMap<i32, usize>,
}

impl TitleitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Titleitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Titleitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Titleitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Titleitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
