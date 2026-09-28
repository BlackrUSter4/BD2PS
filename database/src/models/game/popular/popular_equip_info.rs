use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PopularEquipInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UniqueCharId")]
    pub unique_char_id: Option<i32>,
    #[sqlx(rename = "SlotType")]
    pub slot_type: Option<i32>,
    #[sqlx(rename = "UniqueEquipId")]
    pub unique_equip_id: Option<i32>,
    #[sqlx(rename = "UseCount")]
    pub use_count: Option<i64>,
}