use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ResemaraGachaInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventIndex")]
    pub event_index: Option<i64>,
    #[sqlx(rename = "ResemaraPreviewItemInfoIndex")]
    pub resemara_preview_item_info_index: Option<i64>,
    #[sqlx(rename = "IsLock")]
    pub is_lock: Option<i32>,
}