use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CafeteriaInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "RewardReceiptTime")]
    pub reward_receipt_time: Option<i64>,
    #[sqlx(rename = "SpawnTime")]
    pub spawn_time: Option<i64>,
    #[sqlx(rename = "OngoingManageId")]
    pub ongoing_manage_id: Option<i32>,
    #[sqlx(rename = "DailyRegularCostumeIds")]
    pub daily_regular_costume_ids: Option<String>,
    #[sqlx(rename = "RewardedDailyRegularCostumeIds")]
    pub rewarded_daily_regular_costume_ids: Option<String>,
    #[sqlx(rename = "DailyConnectionCostumeId")]
    pub daily_connection_costume_id: Option<i32>,
    #[sqlx(rename = "CanGetPhoneNumber")]
    pub can_get_phone_number: Option<i32>,
    #[sqlx(rename = "DailyNpcRewardCurrencyCount")]
    pub daily_npc_reward_currency_count: Option<i32>,
    #[sqlx(rename = "IntroStoryRewardClaimed")]
    pub intro_story_reward_claimed: i32,
}