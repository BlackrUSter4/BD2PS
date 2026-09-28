use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ScheduleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ContentId")]
    pub content_id: Option<i32>,
    #[sqlx(rename = "CurrentSeasonIndex")]
    pub current_season: Option<i64>,
    #[sqlx(rename = "NextSeasonIndex")]
    pub next_season: Option<i64>,
}
