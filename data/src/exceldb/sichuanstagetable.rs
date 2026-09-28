// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sichuanstagetable {
    #[serde(rename = "clearTime")]
    pub clear_time: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "limitTime")]
    pub limit_time: Vec<i32>,
    #[serde(rename = "randomBlockGroupID")]
    pub random_block_group_i_d: Vec<i32>,
    #[serde(rename = "randomBlockLimit")]
    pub random_block_limit: Vec<i32>,
    #[serde(rename = "randomBlockTypeCount")]
    pub random_block_type_count: Vec<i32>,
    #[serde(rename = "stageLayoutData")]
    pub stage_layout_data: String,
    #[serde(rename = "touchable")]
    pub touchable: i32,
}

pub struct SichuanstagetableTable {
    records: Vec<Sichuanstagetable>,
    by_id: HashMap<i32, usize>,
}

impl SichuanstagetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Sichuanstagetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Sichuanstagetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Sichuanstagetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Sichuanstagetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
