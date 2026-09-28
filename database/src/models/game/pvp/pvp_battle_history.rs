use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PvpBattleHistory {
    #[sqlx(rename = "BattleIndex")]
    pub battle_index: i64,
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
    #[sqlx(rename = "ContinueWinVp")]
    pub continue_win_vp: Option<i32>,
    #[sqlx(rename = "TimeValue")]
    pub time_value: i64,
    #[sqlx(rename = "IsNoGame")]
    pub is_no_game: bool,
    #[sqlx(rename = "Seed")]
    pub seed: Option<i32>,
    #[sqlx(rename = "DeckSnapshotJson")]
    pub deck_snapshot_json: Option<String>,
}
