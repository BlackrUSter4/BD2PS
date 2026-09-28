// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipmentoptiontable {
    #[serde(rename = "defaultValue")]
    pub default_value: f32,
    #[serde(rename = "getRatio")]
    pub get_ratio: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "growthValue")]
    pub growth_value: Option<f32>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "levelValue")]
    pub level_value: Option<Vec<f32>>,
    #[serde(rename = "rankValue1")]
    pub rank_value1: Option<Vec<f32>>,
    #[serde(rename = "rankValue2")]
    pub rank_value2: Option<Vec<f32>>,
    #[serde(rename = "rankValue3")]
    pub rank_value3: Option<Vec<f32>>,
}

pub struct EquipmentoptiontableTable {
    records: Vec<Equipmentoptiontable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EquipmentoptiontableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipmentoptiontable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Equipmentoptiontable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Equipmentoptiontable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipmentoptiontable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Equipmentoptiontable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
