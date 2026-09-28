// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamesurvivalitemtable {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemPrefabName")]
    pub item_prefab_name: String,
    #[serde(rename = "itemType")]
    pub item_type: Option<i32>,
    #[serde(rename = "itemValue")]
    pub item_value: i32,
    #[serde(rename = "itemValue2")]
    pub item_value2: Option<i32>,
    #[serde(rename = "lifeTime")]
    pub life_time: i32,
}

pub struct FieldminigamesurvivalitemtableTable {
    records: Vec<Fieldminigamesurvivalitemtable>,
    by_id: HashMap<i32, usize>,
}

impl FieldminigamesurvivalitemtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamesurvivalitemtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigamesurvivalitemtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamesurvivalitemtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamesurvivalitemtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
