use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameHubInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Slot")]
    pub slot: Option<i32>,
    #[sqlx(rename = "EventUid")]
    pub event_uid: Option<i32>,
    #[sqlx(rename = "ProgressType")]
    pub progress_type: Option<i32>,
}