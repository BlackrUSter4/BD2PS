// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlrelicmixtable {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "mixMaterialRelicId1", default)]
    pub mix_material_relic_id1: i32,
    #[serde(rename = "mixMaterialRelicId2", default)]
    pub mix_material_relic_id2: i32,
    #[serde(rename = "mixResultRelicId", default)]
    pub mix_result_relic_id: i32,
}

pub struct RlrelicmixtableTable {
    records: Vec<Rlrelicmixtable>,
    by_id: HashMap<i32, usize>,
}

impl RlrelicmixtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlrelicmixtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlrelicmixtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlrelicmixtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlrelicmixtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
