use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ColosseumMatchCandidate {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EnemyOwnerIndex")]
    pub enemy_owner_index: i64,
    #[sqlx(rename = "EnemyUserId")]
    pub enemy_user_id: Option<String>,
    #[sqlx(rename = "EnemyVp")]
    pub enemy_vp: Option<i32>,
    #[sqlx(rename = "EnemyIsBot")]
    pub enemy_is_bot: bool,
    #[sqlx(rename = "EnemyCharIds")]
    pub enemy_char_ids: Option<String>,
    #[sqlx(rename = "CreateTime")]
    pub create_time: Option<i64>,
}
