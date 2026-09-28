use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PackId")]
    pub pack_id: i32,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "StageIndex")]
    pub stage_index: Option<i32>,
    #[sqlx(rename = "Retry")]
    pub retry: Option<i32>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
    #[sqlx(rename = "SeasonHighestPoint")]
    pub season_highest_point: Option<i32>,
    #[sqlx(rename = "IsRewarded")]
    pub is_rewarded: Option<bool>,
    #[sqlx(rename = "StageClearTime")]
    pub stage_clear_time: Option<i32>,
}