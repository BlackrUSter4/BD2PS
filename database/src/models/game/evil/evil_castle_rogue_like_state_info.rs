use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct EvilCastleRogueLikeStateInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Floor")]
    pub floor: Option<i32>,
    #[sqlx(rename = "Room")]
    pub room: Option<i32>,
    #[sqlx(rename = "State")]
    pub state: Option<i32>,
}