use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct StatueGroupInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Season")]
    pub season: Option<i32>,
    #[sqlx(rename = "GroupRank")]
    pub group_rank: Option<i32>,
    #[sqlx(rename = "GuildBaseInfoIndex")]
    pub guild_base_info_index: Option<i64>,
    #[sqlx(rename = "UserStatueInfoIndex")]
    pub user_statue_info_index: Option<String>,
    #[sqlx(rename = "ErrorFlag")]
    pub error_flag: Option<bool>,
}
