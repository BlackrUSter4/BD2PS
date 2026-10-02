// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specialscoutinfotable {
    #[serde(rename = "appearTotalCount", default)]
    pub appear_total_count: i32,
    #[serde(rename = "autoResetMinute", default)]
    pub auto_reset_minute: i32,
    #[serde(rename = "resetCostCount", default)]
    pub reset_cost_count: i32,
    #[serde(rename = "resetCostType", default)]
    pub reset_cost_type: i32,
    #[serde(rename = "resetLimitCount", default)]
    pub reset_limit_count: i32,
}

pub struct SpecialscoutinfotableTable {
    records: Vec<Specialscoutinfotable>,
}

impl SpecialscoutinfotableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Specialscoutinfotable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Specialscoutinfotable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Specialscoutinfotable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
