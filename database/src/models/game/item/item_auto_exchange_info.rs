use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ItemAutoExchangeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OriginalItemType")]
    pub original_item_type: Option<i32>,
    #[sqlx(rename = "OriginalItemId")]
    pub original_item_id: Option<i32>,
    #[sqlx(rename = "OriginalItemCount")]
    pub original_item_count: Option<i32>,
    #[sqlx(rename = "ExchangeItemType")]
    pub exchange_item_type: Option<i32>,
    #[sqlx(rename = "ExchangeItemId")]
    pub exchange_item_id: Option<i32>,
    #[sqlx(rename = "ExchangeItemCount")]
    pub exchange_item_count: Option<i32>,
    #[sqlx(rename = "SortId")]
    pub sort_id: Option<i32>,
}