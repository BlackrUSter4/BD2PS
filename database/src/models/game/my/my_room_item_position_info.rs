use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MyRoomItemPositionInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "ObjectType")]
    pub object_type: Option<i32>,
    #[sqlx(rename = "PositionType")]
    pub position_type: Option<i32>,
    #[sqlx(rename = "X")]
    pub x: Option<i32>,
    #[sqlx(rename = "Y")]
    pub y: Option<i32>,
    #[sqlx(rename = "Rotate")]
    pub rotate: Option<i32>,
    #[sqlx(rename = "Interact")]
    pub interact: Option<i32>,
    #[sqlx(rename = "ItemAnimation")]
    pub item_animation: Option<i32>,
    #[sqlx(rename = "IsWallHidden")]
    pub is_wall_hidden: Option<bool>,
    #[sqlx(rename = "RoomId")]
    pub room_id: i32,
}