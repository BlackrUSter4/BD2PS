// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buffconditiontable {
    #[serde(rename = "attackType")]
    pub attack_type: Option<Vec<i32>>,
    #[serde(rename = "buffGroup")]
    pub buff_group: Option<Vec<i32>>,
    #[serde(rename = "chainLess")]
    pub chain_less: Option<i32>,
    #[serde(rename = "chainMore")]
    pub chain_more: Option<i32>,
    #[serde(rename = "chainMultiple")]
    pub chain_multiple: Option<i32>,
    #[serde(rename = "element")]
    pub element: Option<Vec<i32>>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mainTarget")]
    pub main_target: Option<i32>,
    #[serde(rename = "subTarget")]
    pub sub_target: Option<i32>,
}

pub struct BuffconditiontableTable {
    records: Vec<Buffconditiontable>,
    by_id: HashMap<i32, usize>,
}

impl BuffconditiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Buffconditiontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Buffconditiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Buffconditiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Buffconditiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
