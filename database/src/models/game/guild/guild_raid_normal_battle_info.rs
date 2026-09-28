use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidNormalBattleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: Option<i32>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "CompleteWinCount")]
    pub complete_win_count: Option<i32>,
    #[sqlx(rename = "BattleChallengeIndex")]
    pub battle_challenge_index: i32,
}