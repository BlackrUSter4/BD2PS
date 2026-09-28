use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct OptimizeBaseInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OptimizeIndex")]
    pub optimize_index: Option<i32>,
    #[sqlx(rename = "OptimizeValue")]
    pub optimize_value: Option<bool>,
    #[sqlx(rename = "OptimizeProperty")]
    pub optimize_property: Option<String>,
}