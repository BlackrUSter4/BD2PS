// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profiletexttable {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "textCn")]
    pub text_cn: String,
    #[serde(rename = "textEn")]
    pub text_en: String,
    #[serde(rename = "textJp")]
    pub text_jp: String,
    #[serde(rename = "textTw")]
    pub text_tw: String,
}

pub struct ProfiletexttableTable {
    records: Vec<Profiletexttable>,
    by_id: HashMap<i32, usize>,
}

impl ProfiletexttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Profiletexttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Profiletexttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Profiletexttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Profiletexttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
