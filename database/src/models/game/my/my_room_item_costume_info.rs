use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MyRoomItemCostumeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "UseMyRoomCount")]
    pub use_my_room_count: Option<i32>,
}