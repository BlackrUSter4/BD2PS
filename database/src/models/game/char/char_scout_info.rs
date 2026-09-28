use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharScoutInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "AppearCharId")]
    pub appear_char_id: i32,
    #[sqlx(rename = "UseResetCount")]
    pub use_reset_count: Option<i32>,
    #[sqlx(rename = "NextAutoResetTime")]
    pub next_auto_reset_time: Option<i64>,
    #[sqlx(rename = "ScoutCompleteCharId")]
    pub scout_complete_char_id: i32,
}