use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidPlayInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "BossScore")]
    pub boss_score: Option<i64>,
    #[sqlx(rename = "TotalScore")]
    pub total_score: Option<i32>,
    #[sqlx(rename = "TopPercent")]
    pub top_percent: Option<f64>,
    #[sqlx(rename = "IsPlayRaidToday")]
    pub is_play_raid_today: Option<i32>,
    #[sqlx(rename = "IsNormalBattlePlay")]
    pub is_normal_battle_play: Option<i32>,
    #[sqlx(rename = "BattleMode")]
    pub battle_mode: Option<serde_json::Value>,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
}