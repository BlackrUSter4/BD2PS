use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EventScheduleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "EventType")]
    pub event_type: Option<i32>,
    #[sqlx(rename = "EventId")]
    pub event_id: Option<i32>,
    #[sqlx(rename = "EventSubId")]
    pub event_sub_id: Option<i32>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<i64>,
    #[sqlx(rename = "EndDate")]
    pub end_date: Option<i64>,
    #[sqlx(rename = "IsActive")]
    pub is_active: Option<i32>,
}