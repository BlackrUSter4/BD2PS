use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FriendshipCounselingDaily {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Day")]
    pub day: String,
    #[sqlx(rename = "TotalCount")]
    pub total_count: i32,
    #[sqlx(rename = "CompleteRewardGranted")]
    pub complete_reward_granted: i32,
}
