// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventlisttable {
    #[serde(rename = "actionType", default)]
    pub action_type: Option<String>,
    #[serde(rename = "bgCharIllustName", default)]
    pub bg_char_illust_name: Option<String>,
    #[serde(rename = "eventHubContentIconName", default)]
    pub event_hub_content_icon_name: Option<String>,
    #[serde(rename = "hubContentLocalTextId", default)]
    pub hub_content_local_text_id: i32,
    #[serde(rename = "hubContentType", default)]
    pub hub_content_type: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "slotIndex", default)]
    pub slot_index: i32,
    #[serde(rename = "sortId", default)]
    pub sort_id: Option<i32>,
    #[serde(rename = "groupId", default)]
    pub group_id: Option<i32>,
    #[serde(rename = "hubContentId", default)]
    pub hub_content_id: Option<i32>,
    #[serde(rename = "endDateType", default)]
    pub end_date_type: Option<i32>,
}

pub struct PackeventlisttableTable {
    records: Vec<Packeventlisttable>,
    by_id: HashMap<i32, usize>,
}

impl PackeventlisttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventlisttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Packeventlisttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventlisttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packeventlisttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
