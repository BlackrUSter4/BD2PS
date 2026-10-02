// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualnovelsoundeventtable {
    #[serde(rename = "ambienceName", default)]
    pub ambience_name: Option<String>,
    #[serde(rename = "commonSoundId", default)]
    pub common_sound_id: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "soundEffect", default)]
    pub sound_effect: Option<String>,
}

pub struct VisualnovelsoundeventtableTable {
    records: Vec<Visualnovelsoundeventtable>,
    by_id: HashMap<i32, usize>,
}

impl VisualnovelsoundeventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Visualnovelsoundeventtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Visualnovelsoundeventtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Visualnovelsoundeventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Visualnovelsoundeventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
