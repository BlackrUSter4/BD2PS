use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct AttendanceLimitInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "RewardId")]
    pub reward_id: Option<i32>,
    #[sqlx(rename = "RewardDate")]
    pub reward_date: Option<String>,
}