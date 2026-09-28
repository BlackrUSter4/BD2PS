use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildInitInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "JoinRecvInfoIndex")]
    pub join_recv_info_index: Option<String>,
    #[sqlx(rename = "ActionInfoIndex")]
    pub action_info_index: Option<String>,
    #[sqlx(rename = "IsReward")]
    pub is_reward: Option<i32>,
    #[sqlx(rename = "RaidPlayInfoIndex")]
    pub raid_play_info_index: Option<i64>,
}