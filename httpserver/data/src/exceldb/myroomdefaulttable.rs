// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myroomdefaulttable {
    #[serde(rename = "charLimitCount")]
    pub char_limit_count: i32,
    #[serde(rename = "defaultItemCount")]
    pub default_item_count: i32,
    #[serde(rename = "defaultItemId")]
    pub default_item_id: i32,
    #[serde(rename = "defaultItemType")]
    pub default_item_type: i32,
    #[serde(rename = "otherPlayerListCount")]
    pub other_player_list_count: i32,
    #[serde(rename = "popularLimitCountPercent")]
    pub popular_limit_count_percent: i32,
}

pub struct MyroomdefaulttableTable {
    records: Vec<Myroomdefaulttable>,
}

impl MyroomdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Myroomdefaulttable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Myroomdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Myroomdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
