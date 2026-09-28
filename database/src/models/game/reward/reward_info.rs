use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct RewardInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ItemId")]
    pub item_id: Option<i32>,
    #[sqlx(rename = "ItemType")]
    pub item_type: Option<i32>,
    #[sqlx(rename = "ItemCount")]
    pub item_count: Option<i32>,
}