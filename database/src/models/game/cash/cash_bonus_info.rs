use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CashBonusInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GroupId")]
    pub group_id: i32,
    #[sqlx(rename = "ContentsGroupId")]
    pub contents_group_id: i32,
    #[sqlx(rename = "BuyCount")]
    pub buy_count: i32,
    /// Comma-separated rewarded bonus ids.
    #[sqlx(rename = "RewardedIds")]
    pub rewarded_ids: Option<String>,
}
