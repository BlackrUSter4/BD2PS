// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldboardobjecttable {
    #[serde(rename = "bundleType")]
    pub bundle_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "questId")]
    pub quest_id: i32,
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[serde(rename = "resourceType")]
    pub resource_type: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct FieldboardobjecttableTable {
    records: Vec<Fieldboardobjecttable>,
    by_id: HashMap<i32, usize>,
}

impl FieldboardobjecttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldboardobjecttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldboardobjecttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldboardobjecttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldboardobjecttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
