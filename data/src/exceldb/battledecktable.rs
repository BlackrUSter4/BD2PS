// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Battledecktable {
    #[serde(rename = "battleClearLocalTextId", default)]
    pub battle_clear_local_text_id: i32,
    #[serde(rename = "battleFailLocalTextId", default)]
    pub battle_fail_local_text_id: i32,
    #[serde(rename = "battleFieldBuff", default)]
    pub battle_field_buff: Option<Vec<i32>>,
    #[serde(rename = "bonusPointLocalTextId", default)]
    pub bonus_point_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "bonusPointThumnailLocalTextId", default)]
    pub bonus_point_thumnail_local_text_id: Option<Vec<i32>>,
    #[serde(rename = "bonusPointType", default)]
    pub bonus_point_type: Option<Vec<i32>>,
    #[serde(rename = "bonusPointValue1", default)]
    pub bonus_point_value1: Option<Vec<i32>>,
    #[serde(rename = "bonusPointValue2", default)]
    pub bonus_point_value2: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardCount", default)]
    pub bonus_reward_count: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardId", default)]
    pub bonus_reward_id: Option<Vec<i32>>,
    #[serde(rename = "bonusRewardType", default)]
    pub bonus_reward_type: Option<Vec<i32>>,
    #[serde(rename = "bonusScore", default)]
    pub bonus_score: Option<Vec<i32>>,
    #[serde(rename = "charId", default)]
    pub char_id: Vec<i32>,
    #[serde(rename = "commonSoundId", default)]
    pub common_sound_id: i32,
    #[serde(rename = "costumeId", default)]
    pub costume_id: Vec<i32>,
    #[serde(rename = "costumeId2", default)]
    pub costume_id2: Vec<i32>,
    #[serde(rename = "damageRate", default)]
    pub damage_rate: f32,
    #[serde(rename = "deathTimeStartTurnPvE", default)]
    pub death_time_start_turn_pv_e: i32,
    #[serde(rename = "failWayPointId", default)]
    pub fail_way_point_id: i32,
    #[serde(rename = "healthRate", default)]
    pub health_rate: f32,
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "level", default)]
    pub level: Vec<i32>,
    #[serde(rename = "mapScenePath", default)]
    pub map_scene_path: String,
    #[serde(rename = "position", default)]
    pub position: Vec<i32>,
    #[serde(rename = "sequence", default)]
    pub sequence: Vec<i32>,
    #[serde(rename = "solutionTipTextId", default)]
    pub solution_tip_text_id: Option<i32>,
    #[serde(rename = "useBattleContinue", default)]
    pub use_battle_continue: Option<i32>,
    #[serde(rename = "exp", default)]
    pub exp: Option<i32>,
    #[serde(rename = "rewardCount", default)]
    pub reward_count: Option<Vec<i32>>,
    #[serde(rename = "rewardId", default)]
    pub reward_id: Option<Vec<i32>>,
    #[serde(rename = "rewardSupplyType", default)]
    pub reward_supply_type: Option<i32>,
    #[serde(rename = "rewardType", default)]
    pub reward_type: Option<Vec<i32>>,
    #[serde(rename = "simpleTalkGroupId", default)]
    pub simple_talk_group_id: Option<i32>,
    #[serde(rename = "guideTutorialGroupId", default)]
    pub guide_tutorial_group_id: Option<i32>,
    /// Not a real client-sent field; synthesized at import time from which
    /// server/Data/PACK/<n>/ folder a row came from, since `id` is only
    /// unique within a single pack, not globally. See CLIENT_UPDATE.md's
    /// 2026-10-02 "(packId, id) composite key" entries.
    #[serde(rename = "PackId", default)]
    pub pack_id: i32,
}

pub struct BattledecktableTable {
    records: Vec<Battledecktable>,
    by_id: HashMap<i32, usize>,
    by_pack: HashMap<(i32, i32), usize>,
}

impl BattledecktableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Battledecktable> = serde_json::from_str(&json)?;
        
        let mut by_id = HashMap::with_capacity(records.len());
        let mut by_pack = HashMap::with_capacity(records.len());
        
        for (idx, record) in records.iter().enumerate() {
            by_id.insert(record.id, idx);
            by_pack.insert((record.pack_id, record.id), idx);
        }
        
        Ok(Self {
            records,
            by_id,
            by_pack,
        })
    }

    #[inline]
    pub fn get(&self, id: i32) -> Option<&Battledecktable> {
        self.by_id.get(&id).map(|&idx| &self.records[idx])
    }

    /// Pack-scoped lookup — use this over `get()` for any new call site,
    /// since `id` collides across packs (see `pack_id` field doc).
    #[inline]
    pub fn get_by_pack(&self, pack_id: i32, id: i32) -> Option<&Battledecktable> {
        self.by_pack.get(&(pack_id, id)).map(|&idx| &self.records[idx])
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
