// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlfloortable {
    #[serde(rename = "bossBattleLevelMax")]
    pub boss_battle_level_max: i32,
    #[serde(rename = "bossBattleLevelMin")]
    pub boss_battle_level_min: i32,
    #[serde(rename = "eliteBattleLevelMax")]
    pub elite_battle_level_max: i32,
    #[serde(rename = "eliteBattleLevelMin")]
    pub elite_battle_level_min: i32,
    #[serde(rename = "eventBattleLevelMax")]
    pub event_battle_level_max: i32,
    #[serde(rename = "eventBattleLevelMin")]
    pub event_battle_level_min: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "normalBattleLevelMax")]
    pub normal_battle_level_max: i32,
    #[serde(rename = "normalBattleLevelMin")]
    pub normal_battle_level_min: i32,
    #[serde(rename = "roomGroupId")]
    pub room_group_id: Vec<i32>,
    #[serde(rename = "roomRatio")]
    pub room_ratio: Vec<i32>,
    #[serde(rename = "shopRoomCount")]
    pub shop_room_count: Option<i32>,
    #[serde(rename = "treasureRoomCount")]
    pub treasure_room_count: Option<i32>,
}

pub struct RlfloortableTable {
    records: Vec<Rlfloortable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RlfloortableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlfloortable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlfloortable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Rlfloortable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlfloortable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Rlfloortable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
