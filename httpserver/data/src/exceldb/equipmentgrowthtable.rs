// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipmentgrowthtable {
    #[serde(rename = "breakResultItemCount")]
    pub break_result_item_count: Vec<i32>,
    #[serde(rename = "breakResultItemId")]
    pub break_result_item_id: Vec<i32>,
    #[serde(rename = "breakResultItemType")]
    pub break_result_item_type: Vec<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "growthPoint")]
    pub growth_point: i32,
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "upgradeItemCount")]
    pub upgrade_item_count: Vec<i32>,
    #[serde(rename = "upgradeItemId")]
    pub upgrade_item_id: Vec<i32>,
    #[serde(rename = "upgradeItemType")]
    pub upgrade_item_type: Vec<i32>,
    #[serde(rename = "upgradeSuccessRatio")]
    pub upgrade_success_ratio: Option<f32>,
}

pub struct EquipmentgrowthtableTable {
    records: Vec<Equipmentgrowthtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl EquipmentgrowthtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Equipmentgrowthtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            if let Some(id) = record.id {
                by_id.insert(id, idx);
            }
            by_group.entry(record.group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Equipmentgrowthtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Equipmentgrowthtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Equipmentgrowthtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Equipmentgrowthtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
