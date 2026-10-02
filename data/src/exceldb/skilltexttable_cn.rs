// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkilltexttableCn {
    #[serde(rename = "id", default)]
    pub id: Option<i32>,
    #[serde(rename = "nodeAddTargetBuffTextId", default)]
    pub node_add_target_buff_text_id: Option<Vec<i32>>,
    #[serde(rename = "nodeAddText", default)]
    pub node_add_text: Option<String>,
    #[serde(rename = "targetBuffTextId", default)]
    pub target_buff_text_id: Option<Vec<i32>>,
    #[serde(rename = "text", default)]
    pub text: Option<String>,
}

pub struct SkilltexttableCnTable {
    records: Vec<SkilltexttableCn>,
    by_id: HashMap<i32, usize>,
}

impl SkilltexttableCnTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<SkilltexttableCn> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&SkilltexttableCn> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[SkilltexttableCn] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, SkilltexttableCn> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
