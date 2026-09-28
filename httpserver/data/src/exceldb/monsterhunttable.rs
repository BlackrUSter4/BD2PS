// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monsterhunttable {
    #[serde(rename = "battleDeckId")]
    pub battle_deck_id: i32,
    #[serde(rename = "bossTip2LocalTextId")]
    pub boss_tip2_local_text_id: i32,
    #[serde(rename = "bossTipLocalTextId")]
    pub boss_tip_local_text_id: i32,
    #[serde(rename = "bossUpTimeline")]
    pub boss_up_timeline: String,
    #[serde(rename = "descTextId")]
    pub desc_text_id: i32,
    #[serde(rename = "firstBossTimeline")]
    pub first_boss_timeline: String,
    #[serde(rename = "fixCriticalChain")]
    pub fix_critical_chain: i32,
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
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "monsterHuntChallengeableLevel")]
    pub monster_hunt_challengeable_level: i32,
    #[serde(rename = "monsterHuntRankValueCount")]
    pub monster_hunt_rank_value_count: i32,
    #[serde(rename = "monsterId")]
    pub monster_id: i32,
    #[serde(rename = "openLevelValue")]
    pub open_level_value: i32,
    #[serde(rename = "packId")]
    pub pack_id: i32,
    #[serde(rename = "partsGroupId")]
    pub parts_group_id: i32,
    #[serde(rename = "pointId")]
    pub point_id: i32,
    #[serde(rename = "positionScale")]
    pub position_scale: f32,
    #[serde(rename = "rewardGroupId")]
    pub reward_group_id: i32,
    #[serde(rename = "rewardLevel")]
    pub reward_level: i32,
    #[serde(rename = "stage2Level")]
    pub stage2_level: i32,
    #[serde(rename = "stage2Ratio")]
    pub stage2_ratio: f32,
    #[serde(rename = "stage3Level")]
    pub stage3_level: i32,
    #[serde(rename = "stage3Ratio")]
    pub stage3_ratio: f32,
    #[serde(rename = "startTimeline")]
    pub start_timeline: String,
    #[serde(rename = "statuePackId")]
    pub statue_pack_id: i32,
    #[serde(rename = "teamOpenLevel")]
    pub team_open_level: Vec<i32>,
}

pub struct MonsterhunttableTable {
    records: Vec<Monsterhunttable>,
    by_id: HashMap<i32, usize>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl MonsterhunttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Monsterhunttable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_group.entry(record.parts_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_group,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Monsterhunttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Monsterhunttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Monsterhunttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Monsterhunttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
