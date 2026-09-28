use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldBuffInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BuffId")]
    pub buff_id: Option<i32>,
    #[sqlx(rename = "BuffCount")]
    pub buff_count: Option<i32>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
}