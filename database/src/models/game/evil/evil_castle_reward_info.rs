use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRewardInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EndSeasonInfoIndex")]
    pub end_season_info_index: Option<String>,
    #[sqlx(rename = "EndSeasonTotalInfoIndex")]
    pub end_season_total_info_index: Option<i64>,
    #[sqlx(rename = "RewardInfoBundleIndex")]
    pub reward_info_bundle_index: Option<i64>,
}