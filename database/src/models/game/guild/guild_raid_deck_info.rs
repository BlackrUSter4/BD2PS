use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidDeckInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "DeckInfoIndex")]
    pub deck_info_index: Option<String>,
    #[sqlx(rename = "SupporterDeckInfoIndex")]
    pub supporter_deck_info_index: Option<String>,
    #[sqlx(rename = "IsSupporterDeckUpdate")]
    pub is_supporter_deck_update: Option<i32>,
}