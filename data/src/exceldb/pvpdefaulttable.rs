// Auto-generated from JSON data
// Do not edit manually

use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pvpdefaulttable {
    #[serde(rename = "BattleBGM", default)]
    pub battle_b_g_m: String,
    #[serde(rename = "VP_Min", default)]
    pub v_p_min: i32,
    #[serde(rename = "apBoostMaxCount", default)]
    pub ap_boost_max_count: i32,
    #[serde(rename = "battleEndTurn", default)]
    pub battle_end_turn: i32,
    #[serde(rename = "battleHistoryHour", default)]
    pub battle_history_hour: i32,
    #[serde(rename = "battleHistoryLimitCount", default)]
    pub battle_history_limit_count: i32,
    #[serde(rename = "battleHistoryRefreshCoolTime", default)]
    pub battle_history_refresh_cool_time: i32,
    #[serde(rename = "battlePenaltyRatio", default)]
    pub battle_penalty_ratio: i32,
    #[serde(rename = "battlePenaltyRound", default)]
    pub battle_penalty_round: i32,
    #[serde(rename = "battleReadyTime", default)]
    pub battle_ready_time: i32,
    #[serde(rename = "battleSkipWaitTurn", default)]
    pub battle_skip_wait_turn: i32,
    #[serde(rename = "careCount", default)]
    pub care_count: i32,
    #[serde(rename = "careLose", default)]
    pub care_lose: i32,
    #[serde(rename = "carePointMax", default)]
    pub care_point_max: i32,
    #[serde(rename = "careWin", default)]
    pub care_win: i32,
    #[serde(rename = "correctionFactor", default)]
    pub correction_factor: i32,
    #[serde(rename = "matchCount", default)]
    pub match_count: i32,
    #[serde(rename = "matchExcludeCount", default)]
    pub match_exclude_count: i32,
    #[serde(rename = "matchVpRange", default)]
    pub match_vp_range: Vec<i32>,
    #[serde(rename = "newbieCare", default)]
    pub newbie_care: i32,
    #[serde(rename = "popularCostumeMinRankGroupId", default)]
    pub popular_costume_min_rank_group_id: i32,
    #[serde(rename = "pvpUseCoinCount", default)]
    pub pvp_use_coin_count: i32,
    #[serde(rename = "repeatBattleInterval", default)]
    pub repeat_battle_interval: i32,
    #[serde(rename = "searchingPoolCount", default)]
    pub searching_pool_count: i32,
    #[serde(rename = "startVP", default)]
    pub start_v_p: i32,
    #[serde(rename = "topRankBoundary", default)]
    pub top_rank_boundary: i32,
    #[serde(rename = "topRankMatchVpRange", default)]
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
    pub fn iter(&self) -> std::slice::Iter<'_, Pvpdefaulttable> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}
