use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct UserQuest {
    pub index: i64,
    pub uid: i64,
    pub quest_id: i64,
    pub pack_id: Option<i32>,
    pub status: i32,
    pub progress: i32,
    pub reward_claimed: i32,
    pub last_update: Option<i64>,
}
