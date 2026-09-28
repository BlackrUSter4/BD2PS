// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhythmgamechartable {
    #[serde(rename = "costumeId")]
    pub costume_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemDescNametextId")]
    pub item_desc_nametext_id: i32,
    #[serde(rename = "itemIcon")]
    pub item_icon: String,
    #[serde(rename = "itemTitleNametextId")]
    pub item_title_nametext_id: i32,
}

pub struct RhythmgamechartableTable {
    records: Vec<Rhythmgamechartable>,
    by_id: HashMap<i32, usize>,
}

impl RhythmgamechartableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rhythmgamechartable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rhythmgamechartable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rhythmgamechartable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Rhythmgamechartable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
