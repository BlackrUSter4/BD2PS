use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingItemInfo {
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "ItemId")]
    pub item_id: i32,
    #[sqlx(rename = "ItemType")]
    pub item_type: i32,
    #[sqlx(rename = "Count")]
    pub count: i32,
    #[sqlx(rename = "TimeValue")]
    pub time_value: Option<i64>,
}
