use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct TodayQuestInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "QuestInfoIndex")]
    pub quest_info_index: Option<String>,
    #[sqlx(rename = "ClearQuestIds")]
    pub clear_quest_ids: Option<i32>,
    #[sqlx(rename = "TodayEndTime")]
    pub today_end_time: Option<i64>,
    #[sqlx(rename = "TodayQuestId")]
    pub today_quest_id: i32,
}
