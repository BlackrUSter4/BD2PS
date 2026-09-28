// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packmembertable {
    #[serde(rename = "changeType")]
    pub change_type: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "packType")]
    pub pack_type: Option<i32>,
    #[serde(rename = "subMemberType")]
    pub sub_member_type: Option<i32>,
}

pub struct PackmembertableTable {
    records: Vec<Packmembertable>,
    by_id: HashMap<i32, usize>,
}

impl PackmembertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packmembertable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Packmembertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packmembertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Packmembertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
