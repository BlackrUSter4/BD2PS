use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameSurvivalInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventScheduleId")]
    pub event_schedule_id: Option<i32>,
    #[sqlx(rename = "ActiveCharId")]
    pub active_char_id: i32,
    #[sqlx(rename = "ActiveMapGroupId")]
    pub active_map_group_id: i32,
    #[sqlx(rename = "UserRankScore")]
    pub user_rank_score: Option<i32>,
    #[sqlx(rename = "TopRankOwnerIndex")]
    pub top_rank_owner_index: Option<i64>,
    #[sqlx(rename = "TopRankUserId")]
    pub top_rank_user_id: Option<String>,
    #[sqlx(rename = "TopRankScore")]
    pub top_rank_score: Option<i32>,
    #[sqlx(rename = "StageClearInfoIndex")]
    pub stage_clear_info_index: Option<String>,
    #[sqlx(rename = "CollectionInfoIndex")]
    pub collection_info_index: Option<String>,
    #[sqlx(rename = "UpgradeInfoIndex")]
    pub upgrade_info_index: Option<String>,
}