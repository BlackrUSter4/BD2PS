// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailinfotable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "messageLocalTextId", default)]
    pub message_local_text_id: i32,
    #[serde(rename = "periodDate", default)]
    pub period_date: Option<i32>,
    #[serde(rename = "productLocalTextId", default)]
    pub product_local_text_id: Option<i32>,
    #[serde(rename = "senderLocalTextId", default)]
    pub sender_local_text_id: i32,
    #[serde(rename = "titleLocalTextId", default)]
    pub title_local_text_id: i32,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
}

pub struct MailinfotableTable {
    records: Vec<Mailinfotable>,
    by_id: HashMap<i32, usize>,
}

impl MailinfotableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Mailinfotable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Mailinfotable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Mailinfotable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Mailinfotable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
