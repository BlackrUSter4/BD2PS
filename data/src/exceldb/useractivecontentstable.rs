// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Useractivecontentstable {
    #[serde(rename = "activeDay", default)]
    pub active_day: Option<i32>,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "type", default)]
    pub r#type: Option<i32>,
    #[serde(rename = "typeGroupId", default)]
    pub type_group_id: Option<i32>,
    #[serde(rename = "typeId", default)]
    pub type_id: i32,
    #[serde(rename = "userType", default)]
    pub user_type: Option<i32>,
    #[serde(rename = "contentTicketId", default)]
    pub content_ticket_id: Option<i32>,
    #[serde(rename = "scheduleType", default)]
    pub schedule_type: Option<i32>,
}

pub struct UseractivecontentstableTable {
    records: Vec<Useractivecontentstable>,
    by_id: HashMap<i32, usize>,
}

impl UseractivecontentstableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Useractivecontentstable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Useractivecontentstable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Useractivecontentstable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Useractivecontentstable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
