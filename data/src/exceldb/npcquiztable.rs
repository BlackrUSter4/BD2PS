// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Npcquiztable {
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "npcDistance")]
    pub npc_distance: f32,
    #[serde(rename = "npcId")]
    pub npc_id: i32,
    #[serde(rename = "npcTalkGroupId")]
    pub npc_talk_group_id: i32,
    #[serde(rename = "npcTalkPackId")]
    pub npc_talk_pack_id: i32,
    #[serde(rename = "rewardCount")]
    pub reward_count: Vec<i32>,
    #[serde(rename = "rewardId")]
    pub reward_id: Vec<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Vec<i32>,
    #[serde(rename = "openDelayDays")]
    pub open_delay_days: Option<i32>,
}

pub struct NpcquiztableTable {
    records: Vec<Npcquiztable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl NpcquiztableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Npcquiztable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Npcquiztable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Npcquiztable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Npcquiztable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Npcquiztable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
