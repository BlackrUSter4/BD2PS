// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhythmgamemusictable {
    #[serde(rename = "challengeMaxScore", default)]
    pub challenge_max_score: i32,
    #[serde(rename = "hardMaxScore", default)]
    pub hard_max_score: i32,
    #[serde(rename = "hardNotesName", default)]
    pub hard_notes_name: String,
    #[serde(rename = "hardTotalJudgCount", default)]
    pub hard_total_judg_count: i32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "musicDescLocalTextId", default)]
    pub music_desc_local_text_id: i32,
    #[serde(rename = "musicTitleLocalTextId", default)]
    pub music_title_local_text_id: i32,
    #[serde(rename = "normalMaxScore", default)]
    pub normal_max_score: i32,
    #[serde(rename = "normalNotesName", default)]
    pub normal_notes_name: String,
    #[serde(rename = "normalTotalJudgCount", default)]
    pub normal_total_judg_count: i32,
    #[serde(rename = "playTime", default)]
    pub play_time: i32,
    #[serde(rename = "sortId", default)]
    pub sort_id: i32,
}

pub struct RhythmgamemusictableTable {
    records: Vec<Rhythmgamemusictable>,
    by_id: HashMap<i32, usize>,
}

impl RhythmgamemusictableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rhythmgamemusictable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
        }
        
        Ok(Self {
            records,
            by_id,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Rhythmgamemusictable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rhythmgamemusictable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rhythmgamemusictable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
