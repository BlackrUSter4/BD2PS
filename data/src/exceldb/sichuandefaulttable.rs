// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sichuandefaulttable {
    #[serde(rename = "alertTime")]
    pub alert_time: i32,
    #[serde(rename = "challengeHintTime")]
    pub challenge_hint_time: i32,
    #[serde(rename = "comboBonusFaceillust")]
    pub combo_bonus_faceillust: String,
    #[serde(rename = "comboTimer")]
    pub combo_timer: i32,
    #[serde(rename = "hintTime")]
    pub hint_time: i32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "loadingPrefabName")]
    pub loading_prefab_name: String,
    #[serde(rename = "puzzleHeight")]
    pub puzzle_height: i32,
    #[serde(rename = "puzzleWidth")]
    pub puzzle_width: i32,
    #[serde(rename = "rankMaxCount")]
    pub rank_max_count: i32,
    #[serde(rename = "timerBonus")]
    pub timer_bonus: i32,
}

pub struct SichuandefaulttableTable {
    records: Vec<Sichuandefaulttable>,
    by_id: HashMap<i32, usize>,
}

impl SichuandefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Sichuandefaulttable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Sichuandefaulttable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Sichuandefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Sichuandefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
