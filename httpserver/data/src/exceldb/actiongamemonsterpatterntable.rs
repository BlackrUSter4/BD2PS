// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actiongamemonsterpatterntable {
    #[serde(rename = "attackRangeMin")]
    pub attack_range_min: Option<f32>,
    #[serde(rename = "conditionType1")]
    pub condition_type1: Option<i32>,
    #[serde(rename = "conditionType2")]
    pub condition_type2: Option<i32>,
    #[serde(rename = "conditionValue1")]
    pub condition_value1: Option<i32>,
    #[serde(rename = "conditionValue2")]
    pub condition_value2: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "maxCount")]
    pub max_count: Option<i32>,
    #[serde(rename = "priority")]
    pub priority: i32,
    #[serde(rename = "probability")]
    pub probability: f32,
    #[serde(rename = "skillId")]
    pub skill_id: i32,
}

pub struct ActiongamemonsterpatterntableTable {
    records: Vec<Actiongamemonsterpatterntable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl ActiongamemonsterpatterntableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Actiongamemonsterpatterntable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Actiongamemonsterpatterntable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Actiongamemonsterpatterntable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Actiongamemonsterpatterntable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Actiongamemonsterpatterntable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
