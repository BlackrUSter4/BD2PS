use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct SupporterUsageInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i64>,
    #[sqlx(rename = "BorrowerOwnerIndex")]
    pub borrower_owner_index: Option<i64>,
    #[sqlx(rename = "BorrowerUserId")]
    pub borrower_user_id: Option<String>,
    #[sqlx(rename = "BorrowerPortraitCostumeId")]
    pub borrower_portrait_costume_id: Option<i32>,
    #[sqlx(rename = "BorrowerPortraitDesignId")]
    pub borrower_portrait_design_id: Option<i32>,
    #[sqlx(rename = "BorrowerTitleId")]
    pub borrower_title_id: Option<i32>,
    #[sqlx(rename = "SupporterOwnerIndex")]
    pub supporter_owner_index: Option<i64>,
    #[sqlx(rename = "SupporterSlotIndex")]
    pub supporter_slot_index: Option<i32>,
    #[sqlx(rename = "SupporterCostumeId")]
    pub supporter_costume_id: Option<i32>,
    #[sqlx(rename = "SupporterDesignId")]
    pub supporter_design_id: Option<i32>,
    #[sqlx(rename = "BorrowType")]
    pub borrow_type: Option<i32>,
    #[sqlx(rename = "RewardReceived")]
    pub reward_received: Option<i32>,
    #[sqlx(rename = "UseDate")]
    pub use_date: Option<i64>,
}