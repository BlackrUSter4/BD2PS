use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct TotalWarInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ScoreInfoIndex")]
    pub score_info_index: Option<String>,
    #[sqlx(rename = "TopPercent")]
    pub top_percent: Option<f64>,
    #[sqlx(rename = "TopRankerScore")]
    pub top_ranker_score: Option<i64>,
    #[sqlx(rename = "EngineType")]
    pub engine_type: Option<String>,
    #[sqlx(rename = "ClaimedRewardIds")]
    pub claimed_reward_ids: Option<String>,
}