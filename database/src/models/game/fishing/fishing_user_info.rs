use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct FishingUserInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Exp")]
    pub exp: i32,
    #[sqlx(rename = "Level")]
    pub level: i32,
    #[sqlx(rename = "BoatLevel")]
    pub boat_level: i32,
    #[sqlx(rename = "BoatSkinId")]
    pub boat_skin_id: Option<i32>,
    #[sqlx(rename = "UseRodInvenIndex")]
    pub use_rod_inven_index: Option<i64>,
    #[sqlx(rename = "MultiApResetTime")]
    pub multi_ap_reset_time: Option<i64>,
    #[sqlx(rename = "TrapRewardReceiptTime")]
    pub trap_reward_receipt_time: Option<i64>,
    #[sqlx(rename = "FishInvenSlotCount")]
    pub fish_inven_slot_count: i32,
}

impl Default for FishingUserInfo {
    fn default() -> Self {
        Self {
            uid: 0,
            exp: 0,
            level: 1,
            boat_level: 1,
            boat_skin_id: None,
            use_rod_inven_index: None,
            multi_ap_reset_time: None,
            trap_reward_receipt_time: None,
            fish_inven_slot_count: 20,
        }
    }
}
