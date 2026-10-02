// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Costumepictorialbooktable {
    #[serde(rename = "chaUniqueId", default)]
    pub cha_unique_id: i32,
    #[serde(rename = "collectionBuffId", default)]
    pub collection_buff_id: Vec<i32>,
    #[serde(rename = "costumeID", default)]
    pub costume_i_d: i32,
    #[serde(rename = "enhanceValue", default)]
    pub enhance_value: Vec<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "tabType", default)]
    pub tab_type: i32,
}

pub struct CostumepictorialbooktableTable {
    records: Vec<Costumepictorialbooktable>,
    by_id: HashMap<i32, usize>,
}

impl CostumepictorialbooktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Costumepictorialbooktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Costumepictorialbooktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Costumepictorialbooktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Costumepictorialbooktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
