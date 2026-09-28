use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GachaFixedInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "FixedId")]
    pub fixed_id: Option<i32>,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "Count")]
    pub count: Option<i32>,
    #[sqlx(rename = "ApplySortId")]
    pub apply_sort_id: Option<i32>,
}