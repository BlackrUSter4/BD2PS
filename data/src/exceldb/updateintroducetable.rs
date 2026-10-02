// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Updateintroducetable {
    #[serde(rename = "costumeIllustPath", default)]
    pub costume_illust_path: Option<String>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "magicId", default)]
    pub magic_id: Option<i32>,
    #[serde(rename = "orderId", default)]
    pub order_id: i32,
    #[serde(rename = "resourcePath", default)]
    pub resource_path: String,
    #[serde(rename = "subMagicId", default)]
    pub sub_magic_id: Option<i32>,
    #[serde(rename = "type", default)]
    pub r#type: i32,
}

pub struct UpdateintroducetableTable {
    records: Vec<Updateintroducetable>,
    by_id: HashMap<i32, usize>,
}

impl UpdateintroducetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Updateintroducetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Updateintroducetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Updateintroducetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Updateintroducetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
