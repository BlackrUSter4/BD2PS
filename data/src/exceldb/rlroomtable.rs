// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rlroomtable {
    #[serde(rename = "battleDeckId")]
    pub battle_deck_id: i32,
    #[serde(rename = "battleRecoveryRate")]
    pub battle_recovery_rate: f32,
    #[serde(rename = "battleRewardPoint")]
    pub battle_reward_point: Option<i32>,
    #[serde(rename = "costumeChoiceCount")]
    pub costume_choice_count: Option<i32>,
    #[serde(rename = "eventChoiceId")]
    pub event_choice_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "rewardRelicId")]
    pub reward_relic_id: Option<Vec<i32>>,
    #[serde(rename = "shopTableId")]
    pub shop_table_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
}

pub struct RlroomtableTable {
    records: Vec<Rlroomtable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RlroomtableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rlroomtable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Rlroomtable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Rlroomtable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rlroomtable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rlroomtable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
