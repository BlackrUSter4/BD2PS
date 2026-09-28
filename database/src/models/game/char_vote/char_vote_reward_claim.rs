use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharVoteRewardClaim {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventId")]
    pub event_id: i32,
    #[sqlx(rename = "RewardId")]
    pub reward_id: i32,
}
