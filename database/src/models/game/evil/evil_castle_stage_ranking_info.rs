use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleStageRankingInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
    #[sqlx(rename = "TotalRank")]
    pub total_rank: Option<i32>,
    #[sqlx(rename = "TotalPoint")]
    pub total_point: Option<i32>,
}