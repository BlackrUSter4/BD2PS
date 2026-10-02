// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Newaccountsupplyitemtable {
    #[serde(rename = "supplyItemCount", default)]
    pub supply_item_count: Vec<i32>,
    #[serde(rename = "supplyItemId", default)]
    pub supply_item_id: Vec<i32>,
    #[serde(rename = "supplyItemType", default)]
    pub supply_item_type: Vec<i32>,
}

pub struct NewaccountsupplyitemtableTable {
    records: Vec<Newaccountsupplyitemtable>,
}

impl NewaccountsupplyitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Newaccountsupplyitemtable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Newaccountsupplyitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Newaccountsupplyitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
