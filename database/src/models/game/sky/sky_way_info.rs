use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SkyWayInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "IsAuto")]
    pub is_auto: Option<bool>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "CurrentId")]
    pub current_id: Option<i32>,
    #[sqlx(rename = "MaxClearLevel")]
    pub max_clear_level: Option<i32>,
}