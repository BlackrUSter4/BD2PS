use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MyRoomPresetInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "PresetType")]
    pub preset_type: i32,
    #[sqlx(rename = "Slot")]
    pub slot: i32,
    #[sqlx(rename = "Name")]
    pub name: Option<String>,
    #[sqlx(rename = "SourceOwnerIndex")]
    pub source_owner_index: Option<i64>,
    #[sqlx(rename = "ItemInfoJson")]
    pub item_info_json: Option<String>,
    #[sqlx(rename = "RoomInfoJson")]
    pub room_info_json: Option<String>,
}
