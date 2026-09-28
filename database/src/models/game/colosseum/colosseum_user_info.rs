use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ColosseumUserInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Vp")]
    pub vp: i32,
    #[sqlx(rename = "WinCount")]
    pub win_count: i32,
    #[sqlx(rename = "LoseCount")]
    pub lose_count: i32,
    #[sqlx(rename = "Season")]
    pub season: i32,
    #[sqlx(rename = "ApBuyCount")]
    pub ap_buy_count: i32,
    #[sqlx(rename = "ApBuyResetTime")]
    pub ap_buy_reset_time: Option<i64>,
    #[sqlx(rename = "BattleCountResetTime")]
    pub battle_count_reset_time: Option<i64>,
    #[sqlx(rename = "MatchRerollCount")]
    pub match_reroll_count: i32,
    #[sqlx(rename = "CurrentBattleEnemyIndex")]
    pub current_battle_enemy_index: Option<i64>,
}

impl Default for ColosseumUserInfo {
    fn default() -> Self {
        Self {
            uid: 0,
            vp: 1000,
            win_count: 0,
            lose_count: 0,
            season: 1,
            ap_buy_count: 0,
            ap_buy_reset_time: None,
            battle_count_reset_time: None,
            match_reroll_count: 0,
            current_battle_enemy_index: None,
        }
    }
}
