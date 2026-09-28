use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ContentRankStatueInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Season")]
    pub season: Option<i32>,
    #[sqlx(rename = "ErrorFlag")]
    pub error_flag: Option<bool>,
    #[sqlx(rename = "StatueGroupInfoIndex")]
    pub statue_group_info_index: Option<String>,
}
