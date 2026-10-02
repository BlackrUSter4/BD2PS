// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rhythmgamedefaulttable {
    #[serde(rename = "defaultCharacter", default)]
    pub default_character: i32,
    #[serde(rename = "eventMissionGroupId", default)]
    pub event_mission_group_id: i32,
    #[serde(rename = "feverGaugeTotal", default)]
    pub fever_gauge_total: i32,
    #[serde(rename = "feverMaxTouchableDivider", default)]
    pub fever_max_touchable_divider: i32,
    #[serde(rename = "hardJudgment", default)]
    pub hard_judgment: Vec<i32>,
    #[serde(rename = "hardMissAvailable", default)]
    pub hard_miss_available: i32,
    #[serde(rename = "hardSlidingNoteSpace", default)]
    pub hard_sliding_note_space: i32,
    #[serde(rename = "moreThanChallengeScore", default)]
    pub more_than_challenge_score: i32,
    #[serde(rename = "moreThanComboPivot", default)]
    pub more_than_combo_pivot: Vec<i32>,
    #[serde(rename = "moreThanComboScore", default)]
    pub more_than_combo_score: Vec<i32>,
    #[serde(rename = "nomalJudgment", default)]
    pub nomal_judgment: Vec<i32>,
    #[serde(rename = "normalMissAvailable", default)]
    pub normal_miss_available: i32,
    #[serde(rename = "normalSlidingNoteSpace", default)]
    pub normal_sliding_note_space: i32,
    #[serde(rename = "notesTimingMax", default)]
    pub notes_timing_max: i32,
    #[serde(rename = "notesTimingMin", default)]
    pub notes_timing_min: i32,
    #[serde(rename = "notesTimingSetting", default)]
    pub notes_timing_setting: i32,
    #[serde(rename = "recoveryHpJudgement", default)]
    pub recovery_hp_judgement: i32,
    #[serde(rename = "score", default)]
    pub score: Vec<i32>,
    #[serde(rename = "scoreAccuracy", default)]
    pub score_accuracy: Vec<i32>,
    #[serde(rename = "scoreAccuracyAddingPoint", default)]
    pub score_accuracy_adding_point: Vec<i32>,
}

pub struct RhythmgamedefaulttableTable {
    records: Vec<Rhythmgamedefaulttable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl RhythmgamedefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Rhythmgamedefaulttable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.event_mission_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Rhythmgamedefaulttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Rhythmgamedefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Rhythmgamedefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
