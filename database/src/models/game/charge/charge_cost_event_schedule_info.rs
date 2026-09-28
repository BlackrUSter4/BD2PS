use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ChargeCostEventScheduleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ScheduleIndex")]
    pub schedule_index: Option<i32>,
    #[sqlx(rename = "ItemType")]
    pub item_type: Option<i32>,
    #[sqlx(rename = "MaxCount")]
    pub max_count: Option<i32>,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
}