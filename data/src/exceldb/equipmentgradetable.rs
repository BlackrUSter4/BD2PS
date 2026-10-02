// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipmentgradetable {
    #[serde(rename = "equipGradeLocalTextId", default)]
    pub equip_grade_local_text_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "rankSmeltItemCount", default)]
    pub rank_smelt_item_count: Vec<i32>,
    #[serde(rename = "rankSmeltItemId", default)]
    pub rank_smelt_item_id: Vec<i32>,
    #[serde(rename = "rankSmeltItemType", default)]
    pub rank_smelt_item_type: Vec<i32>,
}

pub struct EquipmentgradetableTable {
    records: Vec<Equipmentgradetable>,
    by_id: HashMap<i32, usize>,
}

impl EquipmentgradetableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipmentgradetable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Equipmentgradetable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipmentgradetable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Equipmentgradetable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
