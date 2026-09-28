use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct AgeGateInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "IsJp")]
    pub is_jp: Option<i32>,
    #[sqlx(rename = "Year")]
    pub year: Option<i32>,
    #[sqlx(rename = "Month")]
    pub month: Option<i32>,
    #[sqlx(rename = "Day")]
    pub day: Option<i32>,
}
