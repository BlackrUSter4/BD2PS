use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidBossBattleInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GuildTotalScore")]
    pub guild_total_score: Option<i64>,
    #[sqlx(rename = "GuildTopPercent")]
    pub guild_top_percent: Option<f64>,
    #[sqlx(rename = "HighestLevel")]
    pub highest_level: Option<i32>,
    #[sqlx(rename = "HighestScore")]
    pub highest_score: Option<i64>,
    #[sqlx(rename = "TopMemberOwnerIndex")]
    pub top_member_owner_index: Option<i64>,
    #[sqlx(rename = "TopMemberUserId")]
    pub top_member_user_id: Option<String>,
    #[sqlx(rename = "TopMemberScore")]
    pub top_member_score: Option<i32>,
}