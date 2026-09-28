// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigameupgradegrouptable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "statType")]
    pub stat_type: i32,
    #[serde(rename = "upgradeDescLocalTextId")]
    pub upgrade_desc_local_text_id: i32,
    #[serde(rename = "upgradeNameTextId")]
    pub upgrade_name_text_id: i32,
    #[serde(rename = "upgradeSpriteName")]
    pub upgrade_sprite_name: String,
}

pub struct FieldminigameupgradegrouptableTable {
    records: Vec<Fieldminigameupgradegrouptable>,
    by_id: HashMap<i32, usize>,
}

impl FieldminigameupgradegrouptableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigameupgradegrouptable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigameupgradegrouptable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigameupgradegrouptable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldminigameupgradegrouptable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
