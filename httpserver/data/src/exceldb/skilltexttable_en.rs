// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkilltexttableEn {
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "nodeAddTargetBuffTextId")]
    pub node_add_target_buff_text_id: Option<Vec<i32>>,
    #[serde(rename = "nodeAddText")]
    pub node_add_text: Option<String>,
    #[serde(rename = "targetBuffTextId")]
    pub target_buff_text_id: Option<Vec<i32>>,
    #[serde(rename = "text")]
    pub text: Option<String>,
}

pub struct SkilltexttableEnTable {
    records: Vec<SkilltexttableEn>,
    by_id: HashMap<i32, usize>,
}

impl SkilltexttableEnTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<SkilltexttableEn> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            if let Some(id) = record.id {
                by_id.insert(id, idx);
            }
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&SkilltexttableEn> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SkilltexttableEn] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<SkilltexttableEn> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
