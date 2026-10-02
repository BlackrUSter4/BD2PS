// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlscoretable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "scoreConditionValue", default)]
    pub score_condition_value: Option<i32>,
    #[serde(rename = "scoreDescNameTextId", default)]
    pub score_desc_name_text_id: i32,
    #[serde(rename = "scoreNameTextId", default)]
    pub score_name_text_id: i32,
    #[serde(rename = "scoreType", default)]
    pub score_type: i32,
    #[serde(rename = "scoreValue", default)]
    pub score_value: Option<i32>,
    #[serde(rename = "scoreValueVeiw", default)]
    pub score_value_veiw: Option<i32>,
}

pub struct RlscoretableTable {
    records: Vec<Rlscoretable>,
    by_id: HashMap<i32, usize>,
}

impl RlscoretableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlscoretable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlscoretable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlscoretable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlscoretable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
