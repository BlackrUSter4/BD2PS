// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idcarddefaulttable {
    #[serde(rename = "defaultItemCount")]
    pub default_item_count: i32,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    #[serde(rename = "defaultItemType")]
    pub default_item_type: i32,
    #[serde(rename = "idCardPresetMaxCount")]
    pub id_card_preset_max_count: i32,
}

pub struct IdcarddefaulttableTable {
    records: Vec<Idcarddefaulttable>,
}

impl IdcarddefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Idcarddefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Idcarddefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Idcarddefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
