// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastletotalinfotable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "lobbyBackGroudImagePath")]
    pub lobby_back_groud_image_path: String,
    #[serde(rename = "lobbyIconPath")]
    pub lobby_icon_path: String,
    #[serde(rename = "lobbyIllustPath")]
    pub lobby_illust_path: String,
}

pub struct EvilcastletotalinfotableTable {
    records: Vec<Evilcastletotalinfotable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastletotalinfotableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastletotalinfotable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastletotalinfotable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastletotalinfotable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Evilcastletotalinfotable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
