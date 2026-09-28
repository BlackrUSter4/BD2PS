// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guildraidbosstable {
    #[serde(rename = "addScore")]
    pub add_score: i32,
    #[serde(rename = "battleMap")]
    pub battle_map: String,
    #[serde(rename = "bossNameTextId")]
    pub boss_name_text_id: i32,
    #[serde(rename = "bossRankNameTextId")]
    pub boss_rank_name_text_id: i32,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "levelUpDamageRate")]
    pub level_up_damage_rate: f32,
    #[serde(rename = "levelUpDamageSlope")]
    pub level_up_damage_slope: f32,
    #[serde(rename = "levelUpHealthRate")]
    pub level_up_health_rate: f32,
    #[serde(rename = "levelUpHealthSlope")]
    pub level_up_health_slope: f32,
    #[serde(rename = "monsterId")]
    pub monster_id: i32,
    #[serde(rename = "partsGroupId")]
    pub parts_group_id: i32,
    #[serde(rename = "positionScale")]
    pub position_scale: f32,
    #[serde(rename = "raidBossBattleDeckId")]
    pub raid_boss_battle_deck_id: i32,
    #[serde(rename = "readyMap")]
    pub ready_map: String,
    #[serde(rename = "rewardCount")]
    pub reward_count: i32,
    #[serde(rename = "rewardType")]
    pub reward_type: i32,
}

pub struct GuildraidbosstableTable {
    records: Vec<Guildraidbosstable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl GuildraidbosstableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Guildraidbosstable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Guildraidbosstable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Guildraidbosstable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Guildraidbosstable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Guildraidbosstable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
