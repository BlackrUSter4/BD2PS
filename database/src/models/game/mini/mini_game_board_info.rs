use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameBoardInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "ScaffoldGroupId")]
    pub scaffold_group_id: Option<i32>,
    #[sqlx(rename = "ScaffoldId")]
    pub scaffold_id: Option<i32>,
    #[sqlx(rename = "CompleteCount")]
    pub complete_count: Option<i32>,
}