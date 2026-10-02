// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packeventbattletable {
    #[serde(rename = "battleDeckId", default)]
    pub battle_deck_id: i32,
    #[serde(rename = "battlePower", default)]
    pub battle_power: i32,
    #[serde(rename = "eventApCount", default)]
    pub event_ap_count: i32,
    #[serde(rename = "groupId", default)]
    pub group_id: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "isFixedCamera", default)]
    pub is_fixed_camera: Option<i32>,
    #[serde(rename = "quickBattlePossible", default)]
    pub quick_battle_possible: Option<i32>,
    #[serde(rename = "repeatRewardCount", default)]
    pub repeat_reward_count: Vec<i32>,
    #[serde(rename = "repeatRewardType", default)]
    pub repeat_reward_type: Vec<i32>,
    #[serde(rename = "repeatRewardid", default)]
    pub repeat_rewardid: Vec<i32>,
}

pub struct PackeventbattletableTable {
    records: Vec<Packeventbattletable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PackeventbattletableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Packeventbattletable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Packeventbattletable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Packeventbattletable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Packeventbattletable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Packeventbattletable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
