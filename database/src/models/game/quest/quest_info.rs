use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct QuestInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Value")]
    pub value: Option<i32>,
    #[sqlx(rename = "ObjectId")]
    pub object_id: i32,
    #[sqlx(rename = "QuestLevel")]
    pub quest_level: Option<i32>,
    #[sqlx(rename = "QuestOpt")]
    pub quest_opt: Option<i32>,
}