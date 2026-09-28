// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldgatetable {
    #[serde(rename = "afterMapId")]
    pub after_map_id: Option<i32>,
    #[serde(rename = "barricadeLocalTextId")]
    pub barricade_local_text_id: Option<i32>,
    #[serde(rename = "beforeMapId")]
    pub before_map_id: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "positionQuestId")]
    pub position_quest_id: Option<i32>,
    #[serde(rename = "questRange")]
    pub quest_range: Option<Vec<i32>>,
    #[serde(rename = "showQuestId")]
    pub show_quest_id: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct FieldgatetableTable {
    records: Vec<Fieldgatetable>,
    by_id: HashMap<i32, usize>,
}

impl FieldgatetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldgatetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldgatetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldgatetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldgatetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
