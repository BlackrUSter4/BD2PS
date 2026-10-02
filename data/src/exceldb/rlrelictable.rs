// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlrelictable {
    #[serde(rename = "grade", default)]
    pub grade: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "relicBuff", default)]
    pub relic_buff: Vec<i32>,
    #[serde(rename = "relicDescTextId", default)]
    pub relic_desc_text_id: i32,
    #[serde(rename = "relicIcon", default)]
    pub relic_icon: String,
    #[serde(rename = "relicIconType", default)]
    pub relic_icon_type: i32,
    #[serde(rename = "relicNameTextId", default)]
    pub relic_name_text_id: i32,
    #[serde(rename = "relicPrice", default)]
    pub relic_price: Option<i32>,
    #[serde(rename = "relicType", default)]
    pub relic_type: Option<i32>,
}

pub struct RlrelictableTable {
    records: Vec<Rlrelictable>,
    by_id: HashMap<i32, usize>,
}

impl RlrelictableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlrelictable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlrelictable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlrelictable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlrelictable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
