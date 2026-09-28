use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CostItemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "Count")]
    pub count: Option<i32>,
    #[sqlx(rename = "KeepFlag")]
    pub keep_flag: Option<i32>,
    #[sqlx(rename = "TimeValue")]
    pub time_value: Option<i64>,
    #[sqlx(rename = "PictorialbookInfoIndex")]
    pub pictorialbook_info_index: Option<i64>,
    #[sqlx(rename = "ExpiryTime")]
    pub expiry_time: Option<i64>,
    #[sqlx(rename = "SortId")]
    pub sort_id: Option<i32>,
    #[sqlx(rename = "UseCount")]
    pub use_count: Option<i32>,
}
