// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamespeedtable {
    #[serde(rename = "boosterSpeed", default)]
    pub booster_speed: f32,
    #[serde(rename = "boosterTime", default)]
    pub booster_time: i32,
    #[serde(rename = "clearTime", default)]
    pub clear_time: i32,
    #[serde(rename = "decreaseValue", default)]
    pub decrease_value: i32,
    #[serde(rename = "ghostPosition", default)]
    pub ghost_position: i32,
    #[serde(rename = "ghostPrefab", default)]
    pub ghost_prefab: String,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "maxCount", default)]
    pub max_count: i32,
    #[serde(rename = "objectGroupId", default)]
    pub object_group_id: i32,
    #[serde(rename = "runCount", default)]
    pub run_count: Vec<i32>,
    #[serde(rename = "runSpeed", default)]
    pub run_speed: Vec<f32>,
    #[serde(rename = "runValue", default)]
    pub run_value: i32,
    #[serde(rename = "startHpPoint", default)]
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
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamespeedtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
