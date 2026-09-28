use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleEndSeasonTotalInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
    #[sqlx(rename = "IsRewarded")]
    pub is_rewarded: Option<bool>,
    #[sqlx(rename = "RewardInfoIndex")]
    pub reward_info_index: Option<String>,
}
