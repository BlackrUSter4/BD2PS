use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PackInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "ClearQuestCount")]
    pub clear_quest_count: Option<i32>,
    #[sqlx(rename = "IsPackComplete")]
    pub is_pack_complete: Option<bool>,
    #[sqlx(rename = "QuestLevel")]
    pub quest_level: Option<i32>,
    #[sqlx(rename = "QuestOpt")]
    pub quest_opt: Option<i32>,
    #[sqlx(rename = "SubQuestCount")]
    pub sub_quest_count: Option<i32>,
    #[sqlx(rename = "ActiveTime")]
    pub active_time: Option<i64>,
    #[sqlx(rename = "IsBuy")]
    pub is_buy: Option<bool>,
}