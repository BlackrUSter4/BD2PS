use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SupporterSlotInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "SlotIndex")]
    pub slot_index: Option<i32>,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: Option<i32>,
    #[sqlx(rename = "Power")]
    pub power: Option<i32>,
    #[sqlx(rename = "BattleUseCount")]
    pub battle_use_count: Option<i32>,
    #[sqlx(rename = "SupporterCharInfo")]
    pub supporter_char_info: Option<String>,
    #[sqlx(rename = "Date")]
    pub date: Option<i64>,
}