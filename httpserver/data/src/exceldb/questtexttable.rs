// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Questtexttable {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "text_cn")]
    pub text_cn: String,
    #[serde(rename = "text_en")]
    pub text_en: String,
    #[serde(rename = "text_jp")]
    pub text_jp: String,
    #[serde(rename = "text_tw")]
    pub text_tw: String,
}

pub struct QuesttexttableTable {
    records: Vec<Questtexttable>,
    by_id: HashMap<i32, usize>,
}

impl QuesttexttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Questtexttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Questtexttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Questtexttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Questtexttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
