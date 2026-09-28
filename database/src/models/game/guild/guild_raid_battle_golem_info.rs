use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildRaidBattleGolemInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "AddExp")]
    pub add_exp: Option<i32>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "Exp")]
    pub exp: Option<i32>,
}