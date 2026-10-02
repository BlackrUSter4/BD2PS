// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Squarestoryarchivingtable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "packId", default)]
    pub pack_id: Vec<i32>,
    #[serde(rename = "storyArchivingTableName", default)]
    pub story_archiving_table_name: Vec<String>,
}

pub struct SquarestoryarchivingtableTable {
    records: Vec<Squarestoryarchivingtable>,
    by_id: HashMap<i32, usize>,
}

impl SquarestoryarchivingtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Squarestoryarchivingtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Squarestoryarchivingtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Squarestoryarchivingtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Squarestoryarchivingtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
