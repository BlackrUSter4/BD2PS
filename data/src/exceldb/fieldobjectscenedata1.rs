// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldobjectscenedata1 {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "objectName", default)]
    pub object_name: String,
    #[serde(rename = "positionX", default)]
    pub position_x: f32,
    #[serde(rename = "positionY", default)]
    pub position_y: f32,
    #[serde(rename = "positionZ", default)]
    pub position_z: f32,
    #[serde(rename = "sceneName", default)]
    pub scene_name: String,
    #[serde(rename = "tableId", default)]
    pub table_id: i32,
    #[serde(rename = "objectType", default)]
    pub object_type: Option<i32>,
}

pub struct Fieldobjectscenedata1Table {
    records: Vec<Fieldobjectscenedata1>,
    by_id: HashMap<i32, usize>,
}

impl Fieldobjectscenedata1Table {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldobjectscenedata1> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldobjectscenedata1> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldobjectscenedata1] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldobjectscenedata1> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
