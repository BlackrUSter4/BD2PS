use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MyRoomUserInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "UserId")]
    pub user_id: Option<String>,
    #[sqlx(rename = "PortraitCostumeId")]
    pub portrait_costume_id: Option<i32>,
    #[sqlx(rename = "PrimaryMyRoomId")]
    pub primary_my_room_id: Option<i32>,
    #[sqlx(rename = "ItemInfoIndex")]
    pub item_info_index: Option<String>,
    #[sqlx(rename = "CostumeInfoIndex")]
    pub costume_info_index: Option<String>,
    #[sqlx(rename = "TrophyInfoIndex")]
    pub trophy_info_index: Option<String>,
    #[sqlx(rename = "MyRoomIndex")]
    pub my_room_index: Option<String>,
    #[sqlx(rename = "MyRoomLikeCount")]
    pub my_room_like_count: Option<i32>,
    #[sqlx(rename = "MyRoomLikeDate")]
    pub my_room_like_date: Option<i64>,
    #[sqlx(rename = "PortraitCostumeDesignId")]
    pub portrait_costume_design_id: Option<i32>,
    #[sqlx(rename = "AllowScopeType")]
    pub allow_scope_type: Option<i32>,
}
