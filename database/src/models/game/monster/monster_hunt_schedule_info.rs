use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntScheduleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "SeasonInfoIndex")]
    pub season_info_index: Option<i64>,
    #[sqlx(rename = "MonsterHuntId")]
    pub monster_hunt_id: Option<i32>,
    #[sqlx(rename = "InfoOpenDay")]
    pub info_open_day: Option<i32>,
    #[sqlx(rename = "CalculateEndDate")]
    pub calculate_end_date: Option<i64>,
    #[sqlx(rename = "ErrorFlag")]
    pub error_flag: Option<i32>,
    #[sqlx(rename = "IndependentFlag")]
    pub independent_flag: Option<i32>,
    #[sqlx(rename = "RankRewardGroupId")]
    pub rank_reward_group_id: Option<i32>,
}