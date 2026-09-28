// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldmonsterregentable {
    #[serde(rename = "genDistance")]
    pub gen_distance: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "nonGenDistance")]
    pub non_gen_distance: Option<i32>,
    #[serde(rename = "questId")]
    pub quest_id: Option<i32>,
    #[serde(rename = "regenSec")]
    pub regen_sec: i32,
    #[serde(rename = "regenType")]
    pub regen_type: Option<i32>,
    #[serde(rename = "resetType")]
    pub reset_type: i32,
}

pub struct FieldmonsterregentableTable {
    records: Vec<Fieldmonsterregentable>,
    by_id: HashMap<i32, usize>,
}

impl FieldmonsterregentableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldmonsterregentable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldmonsterregentable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldmonsterregentable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldmonsterregentable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
