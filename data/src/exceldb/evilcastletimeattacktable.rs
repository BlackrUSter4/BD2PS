// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evilcastletimeattacktable {
    #[serde(rename = "DeductTime", default)]
    pub deduct_time: i32,
    #[serde(rename = "LimitPoint", default)]
    pub limit_point: i32,
    #[serde(rename = "bossExtraPoints", default)]
    pub boss_extra_points: f32,
    #[serde(rename = "damageAddPoint", default)]
    pub damage_add_point: i32,
    #[serde(rename = "damageMaxLimit", default)]
    pub damage_max_limit: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "survivalAddPoint", default)]
    pub survival_add_point: i32,
    #[serde(rename = "turnAddPoint", default)]
    pub turn_add_point: i32,
}

pub struct EvilcastletimeattacktableTable {
    records: Vec<Evilcastletimeattacktable>,
    by_id: HashMap<i32, usize>,
}

impl EvilcastletimeattacktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Evilcastletimeattacktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Evilcastletimeattacktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Evilcastletimeattacktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Evilcastletimeattacktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
