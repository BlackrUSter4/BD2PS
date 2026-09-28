use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct QuestLevelInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PackId")]
    pub pack_id: Option<i32>,
    #[sqlx(rename = "QuestLevel")]
    pub quest_level: Option<i32>,
    #[sqlx(rename = "ClearQuest")]
    pub clear_quest: Option<i32>,
    #[sqlx(rename = "QuestOpt")]
    pub quest_opt: Option<i32>,
    #[sqlx(rename = "IsLevelComplete")]
    pub is_level_complete: Option<bool>,
}