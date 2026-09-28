use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidMemberRankInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Rank")]
    pub rank: Option<i32>,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "UserId")]
    pub user_id: Option<String>,
    #[sqlx(rename = "Score")]
    pub score: Option<i64>,
    #[sqlx(rename = "PortraitCostumeId")]
    pub portrait_costume_id: Option<i32>,
    #[sqlx(rename = "PortraitCostumeDesignId")]
    pub portrait_costume_design_id: Option<i32>,
    #[sqlx(rename = "TitleId")]
    pub title_id: Option<i32>,
    #[sqlx(rename = "OverKillDamage")]
    pub over_kill_damage: Option<i64>,
}