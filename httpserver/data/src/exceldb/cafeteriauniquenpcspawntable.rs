// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cafeteriauniquenpcspawntable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "occurrenceRatio")]
    pub occurrence_ratio: Vec<i32>,
    #[serde(rename = "spawnMaxTime")]
    pub spawn_max_time: i32,
    #[serde(rename = "spawnMinTime")]
    pub spawn_min_time: i32,
    #[serde(rename = "spawnNpcId")]
    pub spawn_npc_id: Vec<i32>,
}

pub struct CafeteriauniquenpcspawntableTable {
    records: Vec<Cafeteriauniquenpcspawntable>,
    by_id: HashMap<i32, usize>,
}

impl CafeteriauniquenpcspawntableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Cafeteriauniquenpcspawntable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Cafeteriauniquenpcspawntable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Cafeteriauniquenpcspawntable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Cafeteriauniquenpcspawntable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
