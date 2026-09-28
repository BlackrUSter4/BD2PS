// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battledecktable {
    #[serde(rename = "battleClearLocalTextId")]
    pub battle_clear_local_text_id: i32,
    #[serde(rename = "battleFailLocalTextId")]
    pub battle_fail_local_text_id: i32,
    #[serde(rename = "battleFieldBuff")]
    pub battle_field_buff: Option<Vec<i32>>,
    #[serde(rename = "bonusPointLocalTextId")]
    pub bonus_point_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "bonusPointThumnailLocalTextId")]
    pub bonus_point_thumnail_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "bonusPointType")]
    pub bonus_point_type: Option<Vec<i32>>,
    #[serde(rename = "bonusPointValue1")]
    pub bonus_point_value1: Option<Vec<i32>>,
    #[serde(rename = "bonusPointValue2")]
    pub bonus_point_value2: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardCount")]
    pub bonus_reward_count: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardId")]
    pub bonus_reward_id: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardType")]
    pub bonus_reward_type: Option<Vec<i32>>,
    #[serde(rename = "bonusScore")]
    pub bonus_score: Option<Vec<i32>>,
    #[serde(rename = "charId")]
    pub char_id: Vec<i32>,
    #[serde(rename = "commonSoundId")]
    pub common_sound_id: i32,
    #[serde(rename = "costumeId")]
    pub costume_id: Vec<i32>,
    #[serde(rename = "costumeId2")]
    pub costume_id2: Vec<i32>,
    #[serde(rename = "damageRate")]
    pub damage_rate: f32,
    #[serde(rename = "deathTimeStartTurnPvE")]
    pub death_time_start_turn_pv_e: i32,
    #[serde(rename = "failWayPointId")]
    pub fail_way_point_id: i32,
    #[serde(rename = "healthRate")]
    pub health_rate: f32,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "level")]
    pub level: Vec<i32>,
    #[serde(rename = "mapScenePath")]
    pub map_scene_path: String,
    #[serde(rename = "position")]
    pub position: Vec<i32>,
    #[serde(rename = "sequence")]
    pub sequence: Vec<i32>,
    #[serde(rename = "solutionTipTextId")]
    pub solution_tip_text_id: Option<i32>,
    #[serde(rename = "useBattleContinue")]
    pub use_battle_continue: Option<i32>,
    #[serde(rename = "exp")]
    pub exp: Option<i32>,
    #[serde(rename = "rewardCount")]
    pub reward_count: Option<Vec<i32>>,
    #[serde(rename = "rewardId")]
    pub reward_id: Option<Vec<i32>>,
    #[serde(rename = "rewardSupplyType")]
    pub reward_supply_type: Option<i32>,
    #[serde(rename = "rewardType")]
    pub reward_type: Option<Vec<i32>>,
    #[serde(rename = "simpleTalkGroupId")]
    pub simple_talk_group_id: Option<i32>,
    #[serde(rename = "guideTutorialGroupId")]
    pub guide_tutorial_group_id: Option<i32>,
}

pub struct BattledecktableTable {
    records: Vec<Battledecktable>,
    by_id: HashMap<i32, usize>,
}

impl BattledecktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Battledecktable> = serde_json::from_str(&json)?;
        
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
    pub fn get(&self, id: i32) -> Option<&Battledecktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Battledecktable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Battledecktable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
