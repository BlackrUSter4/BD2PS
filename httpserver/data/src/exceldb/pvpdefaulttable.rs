// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pvpdefaulttable {
    #[serde(rename = "BattleBGM")]
    pub battle_b_g_m: String,
    #[serde(rename = "VP_Min")]
    pub v_p_min: i32,
    #[serde(rename = "apBoostMaxCount")]
    pub ap_boost_max_count: i32,
    #[serde(rename = "battleEndTurn")]
    pub battle_end_turn: i32,
    #[serde(rename = "battleHistoryHour")]
    pub battle_history_hour: i32,
    #[serde(rename = "battleHistoryLimitCount")]
    pub battle_history_limit_count: i32,
    #[serde(rename = "battleHistoryRefreshCoolTime")]
    pub battle_history_refresh_cool_time: i32,
    #[serde(rename = "battlePenaltyRatio")]
    pub battle_penalty_ratio: i32,
    #[serde(rename = "battlePenaltyRound")]
    pub battle_penalty_round: i32,
    #[serde(rename = "battleReadyTime")]
    pub battle_ready_time: i32,
    #[serde(rename = "battleSkipWaitTurn")]
    pub battle_skip_wait_turn: i32,
    #[serde(rename = "careCount")]
    pub care_count: i32,
    #[serde(rename = "careLose")]
    pub care_lose: i32,
    #[serde(rename = "carePointMax")]
    pub care_point_max: i32,
    #[serde(rename = "careWin")]
    pub care_win: i32,
    #[serde(rename = "correctionFactor")]
    pub correction_factor: i32,
    #[serde(rename = "matchCount")]
    pub match_count: i32,
    #[serde(rename = "matchExcludeCount")]
    pub match_exclude_count: i32,
    #[serde(rename = "matchVpRange")]
    pub match_vp_range: Vec<i32>,
    #[serde(rename = "newbieCare")]
    pub newbie_care: i32,
    #[serde(rename = "popularCostumeMinRankGroupId")]
    pub popular_costume_min_rank_group_id: i32,
    #[serde(rename = "pvpUseCoinCount")]
    pub pvp_use_coin_count: i32,
    #[serde(rename = "repeatBattleInterval")]
    pub repeat_battle_interval: i32,
    #[serde(rename = "searchingPoolCount")]
    pub searching_pool_count: i32,
    #[serde(rename = "startVP")]
    pub start_v_p: i32,
    #[serde(rename = "topRankBoundary")]
    pub top_rank_boundary: i32,
    #[serde(rename = "topRankMatchVpRange")]
    pub top_rank_match_vp_range: Vec<i32>,
}

pub struct PvpdefaulttableTable {
    records: Vec<Pvpdefaulttable>,
    by_group: HashMap<i32, Vec<usize>>,
}

impl PvpdefaulttableTable {
    pub fn load(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let records: Vec<Pvpdefaulttable> = serde_json::from_str(&json)?;
        
        let mut by_group: HashMap<i32, Vec<usize>> = HashMap::new();
        
        for (idx, record) in records.iter().enumerate() {
            by_group.entry(record.popular_costume_min_rank_group_id).or_insert_with(Vec::new).push(idx);
        }
        
        Ok(Self {
            records,
            by_group,
        })
    }

    pub fn by_group(&self, group_id: i32) -> impl Iterator<Item = &Pvpdefaulttable> + '_ {
        self.by_group
            .get(&group_id)
            .into_iter()
            .flat_map(|indices| indices.iter())
            .map(|&idx| &self.records[idx])
    }

    #[inline]
    pub fn all(&self) -> &[Pvpdefaulttable] {
        &self.records
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<Pvpdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
