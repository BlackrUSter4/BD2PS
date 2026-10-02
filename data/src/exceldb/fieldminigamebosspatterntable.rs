// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamebosspatterntable {
    #[serde(rename = "attackCount", default)]
    pub attack_count: i32,
    #[serde(rename = "attackRange", default)]
    pub attack_range: f32,
    #[serde(rename = "attackSpeed", default)]
    pub attack_speed: f32,
    #[serde(rename = "attackType", default)]
    pub attack_type: Option<i32>,
    #[serde(rename = "attackValue", default)]
    pub attack_value: f32,
    #[serde(rename = "chargeTime", default)]
    pub charge_time: f32,
    #[serde(rename = "detectRange", default)]
    pub detect_range: f32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isImmovable", default)]
    pub is_immovable: Option<i32>,
    #[serde(rename = "patternPrefabName", default)]
    pub pattern_prefab_name: Option<String>,
    #[serde(rename = "throwingTrapGroupId", default)]
    pub throwing_trap_group_id: Option<i32>,
}

pub struct FieldminigamebosspatterntableTable {
    records: Vec<Fieldminigamebosspatterntable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamebosspatterntableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamebosspatterntable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigamebosspatterntable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamebosspatterntable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamebosspatterntable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamebosspatterntable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
