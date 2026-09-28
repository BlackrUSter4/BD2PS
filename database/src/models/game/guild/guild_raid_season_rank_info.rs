use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidSeasonRankInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "Score")]
    pub score: Option<i64>,
    #[sqlx(rename = "GuildIndex")]
    pub guild_index: Option<i64>,
    #[sqlx(rename = "GuildName")]
    pub guild_name: Option<String>,
    #[sqlx(rename = "Message")]
    pub message: Option<String>,
    #[sqlx(rename = "Icon")]
    pub icon: Option<i32>,
    #[sqlx(rename = "IconColor")]
    pub icon_color: Option<String>,
    #[sqlx(rename = "FlagGrade")]
    pub flag_grade: Option<i32>,
    #[sqlx(rename = "MemberCount")]
    pub member_count: Option<i32>,
    #[sqlx(rename = "OverKillDamage")]
    pub over_kill_damage: Option<i64>,
}