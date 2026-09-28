use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct AchievementInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "Value")]
    pub value: Option<i64>,
    #[sqlx(rename = "MaxClearId")]
    pub max_clear_id: Option<i32>,
    #[sqlx(rename = "ContentsGroup")]
    pub contents_group: Option<i32>,
}