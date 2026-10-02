// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Randomboxtexttable {
    #[serde(rename = "date", default)]
    pub date: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "text", default)]
    pub text: String,
    #[serde(rename = "textCn", default)]
    pub text_cn: String,
    #[serde(rename = "textEn", default)]
    pub text_en: String,
    #[serde(rename = "textJp", default)]
    pub text_jp: String,
    #[serde(rename = "textTw", default)]
    pub text_tw: String,
}

pub struct RandomboxtexttableTable {
    records: Vec<Randomboxtexttable>,
    by_id: HashMap<i32, usize>,
}

impl RandomboxtexttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Randomboxtexttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Randomboxtexttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Randomboxtexttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Randomboxtexttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
