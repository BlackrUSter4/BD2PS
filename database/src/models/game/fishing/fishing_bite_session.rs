use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingBiteSession {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "FishId")]
    pub fish_id: i32,
    #[sqlx(rename = "Size")]
    pub size: i32,
    #[sqlx(rename = "Hp")]
    pub hp: i32,
    #[sqlx(rename = "Stamina")]
    pub stamina: i32,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
}
