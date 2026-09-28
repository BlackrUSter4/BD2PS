use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MonsterHuntRankInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UserRankInfoIndex")]
    pub user_rank_info_index: Option<String>,
    #[sqlx(rename = "MyRankInfoIndex")]
    pub my_rank_info_index: Option<i64>,
}