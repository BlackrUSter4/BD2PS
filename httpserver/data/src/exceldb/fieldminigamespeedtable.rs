// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamespeedtable {
    #[serde(rename = "boosterSpeed")]
    pub booster_speed: f32,
    #[serde(rename = "boosterTime")]
    pub booster_time: i32,
    #[serde(rename = "clearTime")]
    pub clear_time: i32,
    #[serde(rename = "decreaseValue")]
    pub decrease_value: i32,
    #[serde(rename = "ghostPosition")]
    pub ghost_position: i32,
    #[serde(rename = "ghostPrefab")]
    pub ghost_prefab: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxCount")]
    pub max_count: i32,
    #[serde(rename = "objectGroupId")]
    pub object_group_id: i32,
    #[serde(rename = "runCount")]
    pub run_count: Vec<i32>,
    #[serde(rename = "runSpeed")]
    pub run_speed: Vec<f32>,
    #[serde(rename = "runValue")]
    pub run_value: i32,
    #[serde(rename = "startHpPoint")]
    pub start_hp_point: i32,
}

pub struct FieldminigamespeedtableTable {
    records: Vec<Fieldminigamespeedtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamespeedtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamespeedtable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.object_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Fieldminigamespeedtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamespeedtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamespeedtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Fieldminigamespeedtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
