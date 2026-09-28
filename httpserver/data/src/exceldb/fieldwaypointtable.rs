// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldwaypointtable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isEnableTimeline")]
    pub is_enable_timeline: i32,
    #[serde(rename = "isSafeArea")]
    pub is_safe_area: Option<i32>,
    #[serde(rename = "mapId")]
    pub map_id: Option<i32>,
    #[serde(rename = "questRange")]
    pub quest_range: Vec<i32>,
}

pub struct FieldwaypointtableTable {
    records: Vec<Fieldwaypointtable>,
    by_id: HashMap<i32, usize>,
}

impl FieldwaypointtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldwaypointtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldwaypointtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldwaypointtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldwaypointtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
