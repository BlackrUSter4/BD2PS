use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SupporterDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "UserId")]
    pub user_id: Option<String>,
    #[sqlx(rename = "TitleId")]
    pub title_id: Option<i32>,
    #[sqlx(rename = "PortraitCostumeId")]
    pub portrait_costume_id: Option<i32>,
    #[sqlx(rename = "Greeting")]
    pub greeting: Option<String>,
    #[sqlx(rename = "IsFriend")]
    pub is_friend: Option<i32>,
    #[sqlx(rename = "GuildBaseInfoIndex")]
    pub guild_base_info_index: Option<i64>,
    #[sqlx(rename = "PortraitCostumeDesignId")]
    pub portrait_costume_design_id: Option<i32>,
    #[sqlx(rename = "SupporterSlotsIndex")]
    pub supporter_slots_index: Option<String>,
    #[sqlx(rename = "UsageCount")]
    pub usage_count: Option<i32>,
}