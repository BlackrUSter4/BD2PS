use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MasterTitleInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Name")]
    pub name: Option<String>,
    #[sqlx(rename = "Month")]
    pub month: Option<i32>,
    #[sqlx(rename = "Day")]
    pub day: Option<i32>,
}
