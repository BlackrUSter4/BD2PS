use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CafeteriaRegularCostumeNoteInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: Option<i32>,
    #[sqlx(rename = "ServeCount")]
    pub serve_count: Option<i32>,
    #[sqlx(rename = "IsReceivedReward")]
    pub is_received_reward: Option<bool>,
}