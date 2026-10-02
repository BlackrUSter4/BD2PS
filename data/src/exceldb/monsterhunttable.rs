// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monsterhunttable {
    #[serde(rename = "battleDeckId", default)]
    pub battle_deck_id: i32,
    #[serde(rename = "bossTip2LocalTextId", default)]
    pub boss_tip2_local_text_id: i32,
    #[serde(rename = "bossTipLocalTextId", default)]
    pub boss_tip_local_text_id: i32,
    #[serde(rename = "bossUpTimeline", default)]
    pub boss_up_timeline: String,
    #[serde(rename = "descTextId", default)]
    pub desc_text_id: i32,
    #[serde(rename = "firstBossTimeline", default)]
    pub first_boss_timeline: String,
    #[serde(rename = "fixCriticalChain", default)]
    pub fix_critical_chain: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "levelUpDamageRate", default)]
    pub level_up_damage_rate: f32,
    #[serde(rename = "levelUpDamageSlope", default)]
    pub level_up_damage_slope: f32,
    #[serde(rename = "levelUpHealthRate", default)]
    pub level_up_health_rate: f32,
    #[serde(rename = "levelUpHealthSlope", default)]
    pub level_up_health_slope: f32,
    #[serde(rename = "mapId", default)]
    pub map_id: i32,
    #[serde(rename = "monsterHuntChallengeableLevel", default)]
    pub monster_hunt_challengeable_level: i32,
    #[serde(rename = "monsterHuntRankValueCount", default)]
    pub monster_hunt_rank_value_count: i32,
    #[serde(rename = "monsterId", default)]
    pub monster_id: i32,
    #[serde(rename = "openLevelValue", default)]
    pub open_level_value: i32,
    #[serde(rename = "packId", default)]
    pub pack_id: i32,
    #[serde(rename = "partsGroupId", default)]
    pub parts_group_id: i32,
    #[serde(rename = "pointId", default)]
    pub point_id: i32,
    #[serde(rename = "positionScale", default)]
    pub position_scale: f32,
    #[serde(rename = "rewardGroupId", default)]
    pub reward_group_id: i32,
    #[serde(rename = "rewardLevel", default)]
    pub reward_level: i32,
    #[serde(rename = "stage2Level", default)]
    pub stage2_level: i32,
    #[serde(rename = "stage2Ratio", default)]
    pub stage2_ratio: f32,
    #[serde(rename = "stage3Level", default)]
    pub stage3_level: i32,
    #[serde(rename = "stage3Ratio", default)]
    pub stage3_ratio: f32,
    #[serde(rename = "startTimeline", default)]
    pub start_timeline: String,
    #[serde(rename = "statuePackId", default)]
    pub statue_pack_id: i32,
    #[serde(rename = "teamOpenLevel", default)]
    pub team_open_level: Vec<i32>,
    #[serde(rename = "spStartHunterCount", default)]
    pub sp_start_hunter_count: Option<i32>,
    #[serde(rename = "spTurnAddHunterCount", default)]
    pub sp_turn_add_hunter_count: Option<i32>,
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
    pub fn iter(&self) -> std::slice::Iter<'_, Monsterhunttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
