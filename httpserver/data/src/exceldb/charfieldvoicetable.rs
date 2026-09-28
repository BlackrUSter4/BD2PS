// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charfieldvoicetable {
    #[serde(rename = "damageVoiceName")]
    pub damage_voice_name: Option<String>,
    #[serde(rename = "dashVoiceName")]
    pub dash_voice_name: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "liftVoiceName")]
    pub lift_voice_name: Option<String>,
}

pub struct CharfieldvoicetableTable {
    records: Vec<Charfieldvoicetable>,
    by_id: HashMap<i32, usize>,
}

impl CharfieldvoicetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Charfieldvoicetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Charfieldvoicetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Charfieldvoicetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Charfieldvoicetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
