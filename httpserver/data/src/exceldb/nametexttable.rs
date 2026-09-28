// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nametexttable {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[serde(rename = "text_cn")]
    pub text_cn: Option<String>,
    #[serde(rename = "text_en")]
    pub text_en: Option<String>,
    #[serde(rename = "text_jp")]
    pub text_jp: Option<String>,
    #[serde(rename = "text_tw")]
    pub text_tw: Option<String>,
}

pub struct NametexttableTable {
    records: Vec<Nametexttable>,
    by_id: HashMap<i32, usize>,
}

impl NametexttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Nametexttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Nametexttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Nametexttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Nametexttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
