// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamekeybuttontable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "skillButton1")]
    pub skill_button1: i32,
    #[serde(rename = "skillButton1Hold")]
    pub skill_button1_hold: Option<i32>,
    #[serde(rename = "skillButton2")]
    pub skill_button2: i32,
    #[serde(rename = "skillButton3")]
    pub skill_button3: i32,
    #[serde(rename = "skillButton4")]
    pub skill_button4: i32,
    #[serde(rename = "skillButton5")]
    pub skill_button5: i32,
}

pub struct ActiongamekeybuttontableTable {
    records: Vec<Actiongamekeybuttontable>,
    by_id: HashMap<i32, usize>,
}

impl ActiongamekeybuttontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamekeybuttontable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamekeybuttontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamekeybuttontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Actiongamekeybuttontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
