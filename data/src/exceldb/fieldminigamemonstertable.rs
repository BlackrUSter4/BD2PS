// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fieldminigamemonstertable {
    #[serde(rename = "attackPatternGroupId")]
    pub attack_pattern_group_id: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "growthValue")]
    pub growth_value: Option<f32>,
    #[serde(rename = "health")]
    pub health: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "itemBoxId")]
    pub item_box_id: Option<Vec<i32>>,
    #[serde(rename = "monsterPrefabName")]
    pub monster_prefab_name: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f32,
    #[serde(rename = "patternCooldown")]
    pub pattern_cooldown: Option<f32>,
    #[serde(rename = "powerValue")]
    pub power_value: f32,
    #[serde(rename = "repeatCount")]
    pub repeat_count: i32,
    #[serde(rename = "spawnCount")]
    pub spawn_count: i32,
    #[serde(rename = "spawnDelayTime")]
    pub spawn_delay_time: Option<f32>,
    #[serde(rename = "spawnDuration")]
    pub spawn_duration: Option<i32>,
    #[serde(rename = "spawnTime")]
    pub spawn_time: Option<i32>,
    #[serde(rename = "startSpawnTime")]
    pub start_spawn_time: Option<i32>,
    #[serde(rename = "type")]
    pub r#type: Option<i32>,
}

pub struct FieldminigamemonstertableTable {
    records: Vec<Fieldminigamemonstertable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl FieldminigamemonstertableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Fieldminigamemonstertable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Fieldminigamemonstertable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Fieldminigamemonstertable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Fieldminigamemonstertable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Fieldminigamemonstertable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
