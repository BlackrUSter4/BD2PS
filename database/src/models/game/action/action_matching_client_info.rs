use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ActionMatchingClientInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "UserInfoIndex")]
    pub user_info_index: Option<i64>,
    #[sqlx(rename = "IsRoomMaster")]
    pub is_room_master: Option<i32>,
    #[sqlx(rename = "EnterTime")]
    pub enter_time: Option<i64>,
    #[sqlx(rename = "Guid")]
    pub guid: Option<String>,
    #[sqlx(rename = "ActionChar")]
    pub action_char: Option<i32>,
}