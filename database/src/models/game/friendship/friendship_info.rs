use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FriendshipInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: i32,
    #[sqlx(rename = "Level")]
    pub level: i32,
    #[sqlx(rename = "Exp")]
    pub exp: i32,
    #[sqlx(rename = "LastCounselingDate")]
    pub last_counseling_date: Option<i64>,
}

impl FriendshipInfo {
    pub fn new_default(uid: i64, costume_id: i32) -> Self {
        Self {
            uid,
            costume_id,
            level: 1,
            exp: 0,
            last_counseling_date: None,
        }
    }
}
