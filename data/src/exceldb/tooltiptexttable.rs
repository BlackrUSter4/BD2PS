// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltiptexttable {
    #[serde(rename = "date", default)]
    pub date: Option<String>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "text", default)]
    pub text: Option<String>,
    #[serde(rename = "text_cn", default)]
    pub text_cn: Option<String>,
    #[serde(rename = "text_en", default)]
    pub text_en: Option<String>,
    #[serde(rename = "text_jp", default)]
    pub text_jp: Option<String>,
    #[serde(rename = "text_tw", default)]
    pub text_tw: Option<String>,
}

pub struct TooltiptexttableTable {
    records: Vec<Tooltiptexttable>,
    by_id: HashMap<i32, usize>,
}

impl TooltiptexttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Tooltiptexttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Tooltiptexttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Tooltiptexttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Tooltiptexttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
