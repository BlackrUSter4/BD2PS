use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ColosseumBattleHistory {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Season")]
    pub season: i32,
    #[sqlx(rename = "IsAttacker")]
    pub is_attacker: bool,
    #[sqlx(rename = "EnemyOwnerIndex")]
    pub enemy_owner_index: Option<i64>,
    #[sqlx(rename = "EnemyUserId")]
    pub enemy_user_id: Option<String>,
    #[sqlx(rename = "EnemyVp")]
    pub enemy_vp: Option<i32>,
    #[sqlx(rename = "EnemyRank")]
    pub enemy_rank: Option<i32>,
    #[sqlx(rename = "ChangeVp")]
    pub change_vp: Option<i32>,
    #[sqlx(rename = "TimeValue")]
    pub time_value: Option<i64>,
    #[sqlx(rename = "EnemyTopPercent")]
    pub enemy_top_percent: Option<f64>,
    #[sqlx(rename = "IsNoGame")]
    pub is_no_game: bool,
}
