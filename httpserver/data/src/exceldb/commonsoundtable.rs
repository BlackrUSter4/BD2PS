// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commonsoundtable {
    #[serde(rename = "bgmPlayerHidden")]
    pub bgm_player_hidden: Option<i32>,
    #[serde(rename = "contentTicketId")]
    pub content_ticket_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "packId")]
    pub pack_id: Option<i32>,
    #[serde(rename = "sortId")]
    pub sort_id: Option<i32>,
    #[serde(rename = "soundNameTextId")]
    pub sound_name_text_id: Option<i32>,
    #[serde(rename = "soundPath")]
    pub sound_path: String,
    #[serde(rename = "soundSourceNameTextId")]
    pub sound_source_name_text_id: Option<i32>,
}

pub struct CommonsoundtableTable {
    records: Vec<Commonsoundtable>,
    by_id: HashMap<i32, usize>,
}

impl CommonsoundtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Commonsoundtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Commonsoundtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Commonsoundtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Commonsoundtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
