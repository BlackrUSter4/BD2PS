use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct RelayServerInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Region")]
    pub region: Option<String>,
    #[sqlx(rename = "Ip")]
    pub ip: Option<String>,
    #[sqlx(rename = "Port")]
    pub port: Option<i32>,
}