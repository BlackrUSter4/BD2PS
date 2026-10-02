// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gachavisualtable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "visualLocalTextID", default)]
    pub visual_local_text_i_d: i32,
    #[serde(rename = "voiceResourceName", default)]
    pub voice_resource_name: Option<String>,
}

pub struct GachavisualtableTable {
    records: Vec<Gachavisualtable>,
    by_id: HashMap<i32, usize>,
}

impl GachavisualtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Gachavisualtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Gachavisualtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Gachavisualtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Gachavisualtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
