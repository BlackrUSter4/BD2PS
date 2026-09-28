// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltiptextdescriptiontable {
    #[serde(rename = "descTextId")]
    pub desc_text_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "titleTextId")]
    pub title_text_id: i32,
}

pub struct TooltiptextdescriptiontableTable {
    records: Vec<Tooltiptextdescriptiontable>,
    by_id: HashMap<i32, usize>,
}

impl TooltiptextdescriptiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Tooltiptextdescriptiontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Tooltiptextdescriptiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Tooltiptextdescriptiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Tooltiptextdescriptiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
