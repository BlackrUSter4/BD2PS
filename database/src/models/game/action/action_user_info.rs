use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ActionUserInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "ActionCharId")]
    pub action_char_id: Option<i32>,
    #[sqlx(rename = "ClientState")]
    pub client_state: Option<i32>,
    #[sqlx(rename = "NetworkState")]
    pub network_state: Option<i32>,
    #[sqlx(rename = "NetworkPing")]
    pub network_ping: Option<i32>,
    #[sqlx(rename = "CalcState")]
    pub calc_state: Option<i32>,
}