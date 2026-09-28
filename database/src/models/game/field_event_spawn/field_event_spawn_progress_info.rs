use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldEventSpawnProgressInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "SpawnEventId")]
    pub spawn_event_id: Option<i32>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    /// JSON-encoded Vec<(spawn_event_id, monster_group_id, monster_id)>.
    #[sqlx(rename = "CaughtInfo")]
    pub caught_info: Option<String>,
}
