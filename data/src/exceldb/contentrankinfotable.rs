// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contentrankinfotable {
    #[serde(rename = "evilCastleFloorMyRankMaxCount", default)]
    pub evil_castle_floor_my_rank_max_count: i32,
    #[serde(rename = "evilCastleFloorTopRankMaxCount", default)]
    pub evil_castle_floor_top_rank_max_count: i32,
    #[serde(rename = "evilCastleTotalMyRankMaxCount", default)]
    pub evil_castle_total_my_rank_max_count: i32,
    #[serde(rename = "evilCastleTotalTopRankMaxCount", default)]
    pub evil_castle_total_top_rank_max_count: i32,
    #[serde(rename = "mirrorWarMyRankMaxCount", default)]
    pub mirror_war_my_rank_max_count: i32,
    #[serde(rename = "mirrorWarTopRankMaxCount", default)]
    pub mirror_war_top_rank_max_count: i32,
    #[serde(rename = "monsterHuntMyRankMaxCount", default)]
    pub monster_hunt_my_rank_max_count: i32,
    #[serde(rename = "monsterHuntTopRankMaxCount", default)]
    pub monster_hunt_top_rank_max_count: i32,
    #[serde(rename = "roguelikeMyRankMaxCount", default)]
    pub roguelike_my_rank_max_count: i32,
    #[serde(rename = "roguelikeTopRankMaxCount", default)]
    pub roguelike_top_rank_max_count: i32,
}

pub struct ContentrankinfotableTable {
    records: Vec<Contentrankinfotable>,
}

impl ContentrankinfotableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Contentrankinfotable> = serde_json::from_str(&json)?;
        
        Ok(Self {
            records,
        })
    }

    #[inline]
    pub fn all(&self) -> &[Contentrankinfotable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Contentrankinfotable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
