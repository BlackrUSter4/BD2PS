use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SpineInteractionRewardClaim {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InteractionGroupId")]
    pub interaction_group_id: i32,
    #[sqlx(rename = "GroupId")]
    pub group_id: i32,
    #[sqlx(rename = "Id")]
    pub id: i32,
}
