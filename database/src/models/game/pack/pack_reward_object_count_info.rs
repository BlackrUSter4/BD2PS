use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PackRewardObjectCountInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "PackId")]
    pub pack_id: Option<i32>,
    #[sqlx(rename = "Count")]
    pub count: Option<i32>,
    #[sqlx(rename = "MaxCount")]
    pub max_count: Option<i32>,
}