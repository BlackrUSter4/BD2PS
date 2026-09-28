use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GachaLogInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "GachaGroupId")]
    pub gacha_group_id: Option<i32>,
    #[sqlx(rename = "GachaId")]
    pub gacha_id: Option<i32>,
    #[sqlx(rename = "BuyType")]
    pub buy_type: Option<serde_json::Value>,
    #[sqlx(rename = "GachaCount")]
    pub gacha_count: Option<i32>,
    #[sqlx(rename = "GetPoint")]
    pub get_point: Option<i32>,
    #[sqlx(rename = "GachaType")]
    pub gacha_type: Option<serde_json::Value>,
    #[sqlx(rename = "PickupItemId")]
    pub pickup_item_id: Option<i32>,
    #[sqlx(rename = "GachaFixedInfoIndex")]
    pub gacha_fixed_info_index: Option<String>,
    #[sqlx(rename = "RewardInfoBundleIndex")]
    pub reward_info_bundle_index: Option<i64>,
    #[sqlx(rename = "NewSort")]
    pub new_sort: i32,
    #[sqlx(rename = "SelectionSort")]
    pub selection_sort: i32,
    #[sqlx(rename = "DecreaseItemInfoIndex")]
    pub decrease_item_info_index: Option<String>,
    #[sqlx(rename = "LogTime")]
    pub log_time: Option<i64>,
}