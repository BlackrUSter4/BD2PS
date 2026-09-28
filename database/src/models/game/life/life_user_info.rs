use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct LifeUserInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "LifeCoin")]
    pub life_coin: i32,
    #[sqlx(rename = "LifeWorldId")]
    pub life_world_id: Option<i32>,
    #[sqlx(rename = "LoggingLevel")]
    pub logging_level: i32,
    #[sqlx(rename = "LoggingExp")]
    pub logging_exp: i32,
    #[sqlx(rename = "MiningLevel")]
    pub mining_level: i32,
    #[sqlx(rename = "MiningExp")]
    pub mining_exp: i32,
    #[sqlx(rename = "FarmingLevel")]
    pub farming_level: i32,
    #[sqlx(rename = "FarmingExp")]
    pub farming_exp: i32,
}

impl Default for LifeUserInfo {
    fn default() -> Self {
        Self {
            uid: 0,
            life_coin: 0,
            life_world_id: None,
            logging_level: 1,
            logging_exp: 0,
            mining_level: 1,
            mining_exp: 0,
            farming_level: 1,
            farming_exp: 0,
        }
    }
}
