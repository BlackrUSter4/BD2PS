use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRankingInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UserRankingInfoIndex")]
    pub user_ranking_info_index: Option<String>,
    #[sqlx(rename = "MyRankingInfoIndex")]
    pub my_ranking_info_index: Option<i64>,
}