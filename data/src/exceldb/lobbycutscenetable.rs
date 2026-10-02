// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lobbycutscenetable {
    #[serde(rename = "costumeConceptInfoTextEnumCount", default)]
    pub costume_concept_info_text_enum_count: Vec<i32>,
    #[serde(rename = "costumeConceptInfoTextType", default)]
    pub costume_concept_info_text_type: Vec<i32>,
    #[serde(rename = "costumeConceptInfoVoiceEnumCount", default)]
    pub costume_concept_info_voice_enum_count: Vec<i32>,
    #[serde(rename = "costumeConceptInfoVoiceType", default)]
    pub costume_concept_info_voice_type: Vec<i32>,
    #[serde(rename = "costumeDesignId", default)]
    pub costume_design_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
}

pub struct LobbycutscenetableTable {
    records: Vec<Lobbycutscenetable>,
    by_id: HashMap<i32, usize>,
}

impl LobbycutscenetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Lobbycutscenetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Lobbycutscenetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Lobbycutscenetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Lobbycutscenetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
