use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FieldEventSpawnDailyCount {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Date")]
    pub date: String,
    #[sqlx(rename = "NormalCount")]
    pub normal_count: i32,
    #[sqlx(rename = "SpecialCount")]
    pub special_count: i32,
}
