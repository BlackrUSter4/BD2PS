// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eventtable {
    #[serde(rename = "bannerFontLocalTextId")]
    pub banner_font_local_text_id: i32,
    #[serde(rename = "bannerResouceName")]
    pub banner_resouce_name: String,
    #[serde(rename = "categoryBanner")]
    pub category_banner: String,
    #[serde(rename = "coinExchangeType")]
    pub coin_exchange_type: Option<i32>,
    #[serde(rename = "eventId")]
    pub event_id: i32,
    #[serde(rename = "eventType")]
    pub event_type: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isActiveOnEnter")]
    pub is_active_on_enter: Option<i32>,
    #[serde(rename = "packageConnect")]
    pub package_connect: Option<String>,
    #[serde(rename = "sortId")]
    pub sort_id: Option<i32>,
}

pub struct EventtableTable {
    records: Vec<Eventtable>,
    by_id: HashMap<i32, usize>,
}

impl EventtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Eventtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Eventtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Eventtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Eventtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
