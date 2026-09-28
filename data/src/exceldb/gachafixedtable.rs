// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gachafixedtable {
    #[serde(rename = "g3EquipFixedCount")]
    pub g3_equip_fixed_count: Option<i32>,
    #[serde(rename = "g4CostumeFixedCount")]
    pub g4_costume_fixed_count: Option<i32>,
    #[serde(rename = "g4EquipFixedCount")]
    pub g4_equip_fixed_count: Option<i32>,
    #[serde(rename = "g5CostumeFixedCount")]
    pub g5_costume_fixed_count: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "isResetFixedCount")]
    pub is_reset_fixed_count: Option<i32>,
}

pub struct GachafixedtableTable {
    records: Vec<Gachafixedtable>,
    by_id: HashMap<i32, usize>,
}

impl GachafixedtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Gachafixedtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Gachafixedtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Gachafixedtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Gachafixedtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
