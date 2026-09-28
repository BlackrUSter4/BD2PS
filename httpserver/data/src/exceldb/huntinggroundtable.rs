// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Huntinggroundtable {
    #[serde(rename = "bossId")]
    pub boss_id: i32,
    #[serde(rename = "difficulty")]
    pub difficulty: Option<i32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "monsterId")]
    pub monster_id: Vec<i32>,
    #[serde(rename = "recommendGrowthPoint")]
    pub recommend_growth_point: i32,
}

pub struct HuntinggroundtableTable {
    records: Vec<Huntinggroundtable>,
    by_id: HashMap<i32, usize>,
}

impl HuntinggroundtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Huntinggroundtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Huntinggroundtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Huntinggroundtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Huntinggroundtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
