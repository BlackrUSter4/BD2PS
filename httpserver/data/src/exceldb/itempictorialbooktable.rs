// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Itempictorialbooktable {
    #[serde(rename = "category")]
    pub category: Option<i32>,
    #[serde(rename = "collectionBuffId")]
    pub collection_buff_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "packId")]
    pub pack_id: i32,
}

pub struct ItempictorialbooktableTable {
    records: Vec<Itempictorialbooktable>,
    by_id: HashMap<i32, usize>,
}

impl ItempictorialbooktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Itempictorialbooktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Itempictorialbooktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Itempictorialbooktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Itempictorialbooktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
