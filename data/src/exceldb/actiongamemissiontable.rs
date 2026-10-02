// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamemissiontable {
    #[serde(rename = "conditionSubType", default)]
    pub condition_sub_type: i32,
    #[serde(rename = "conditionSubTypeParam", default)]
    pub condition_sub_type_param: Option<i32>,
    #[serde(rename = "conditionType", default)]
    pub condition_type: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "missionDescNameTextId", default)]
    pub mission_desc_name_text_id: i32,
    #[serde(rename = "missionNameNameTextId", default)]
    pub mission_name_name_text_id: i32,
    #[serde(rename = "monsterType", default)]
    pub monster_type: i32,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Option<i32>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Option<i32>,
    #[serde(rename = "rewardType", default)]
    pub reward_type: Option<i32>,
}

pub struct ActiongamemissiontableTable {
    records: Vec<Actiongamemissiontable>,
    by_id: HashMap<i32, usize>,
}

impl ActiongamemissiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamemissiontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamemissiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamemissiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Actiongamemissiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
