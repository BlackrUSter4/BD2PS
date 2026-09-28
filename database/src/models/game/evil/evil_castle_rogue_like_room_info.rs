use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRogueLikeRoomInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    /// Which floor this room belongs to — added because the original
    /// migration had no way to scope rooms to a floor at all.
    #[sqlx(rename = "Floor")]
    pub floor: i32,
    #[sqlx(rename = "Number")]
    pub number: Option<i32>,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "IsClear")]
    pub is_clear: Option<i32>,
}