// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questtitletable {
    #[serde(rename = "EndquestId")]
    pub endquest_id: i32,
    #[serde(rename = "StartquestId")]
    pub startquest_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "questStartInfoQuestTextId")]
    pub quest_start_info_quest_text_id: i32,
    #[serde(rename = "questTitleQuestTextId")]
    pub quest_title_quest_text_id: i32,
}

pub struct QuesttitletableTable {
    records: Vec<Questtitletable>,
    by_id: HashMap<i32, usize>,
}

impl QuesttitletableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questtitletable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Questtitletable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questtitletable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Questtitletable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
