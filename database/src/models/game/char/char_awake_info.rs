use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharAwakeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UniqueCharId")]
    pub unique_char_id: Option<i32>,
    #[sqlx(rename = "IsAwake")]
    pub is_awake: Option<bool>,
    #[sqlx(rename = "ImprintSlot1Level")]
    pub imprint_slot_1_level: Option<i32>,
    #[sqlx(rename = "ImprintSlot2Level")]
    pub imprint_slot_2_level: Option<i32>,
    #[sqlx(rename = "ImprintSlot3Level")]
    pub imprint_slot_3_level: Option<i32>,
}