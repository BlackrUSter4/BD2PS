use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleHistoryInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BattleIndex")]
    pub battle_index: Option<i64>,
    #[sqlx(rename = "BattleResult")]
    pub battle_result: Option<i32>,
    #[sqlx(rename = "EnemyOwnerIndex")]
    pub enemy_owner_index: Option<i64>,
    #[sqlx(rename = "EnemyUserId")]
    pub enemy_user_id: Option<String>,
    #[sqlx(rename = "EnemyVp")]
    pub enemy_vp: Option<i32>,
    #[sqlx(rename = "EnemyRank")]
    pub enemy_rank: Option<i32>,
    #[sqlx(rename = "EnemyCostumeId")]
    pub enemy_costume_id: i64,
    #[sqlx(rename = "EnemyCostumeDesignId")]
    pub enemy_costume_design_id: i64,
    #[sqlx(rename = "ChangeVp")]
    pub change_vp: Option<i32>,
    #[sqlx(rename = "ContinueWinVp")]
    pub continue_win_vp: Option<i32>,
    #[sqlx(rename = "TimeValue")]
    pub time_value: Option<i64>,
    #[sqlx(rename = "IsNoGame")]
    pub is_no_game: Option<i32>,
}