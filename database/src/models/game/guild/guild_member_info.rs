use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildMemberInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i64>,
    #[sqlx(rename = "OwnerIndex")]
    pub owner_index: Option<i64>,
    #[sqlx(rename = "UserId")]
    pub user_id: Option<String>,
    #[sqlx(rename = "TitleId")]
    pub title_id: Option<i32>,
    #[sqlx(rename = "PortraitCostumeId")]
    pub portrait_costume_id: Option<i32>,
    #[sqlx(rename = "PortraitCostumeDesignId")]
    pub portrait_costume_design_id: Option<i32>,
    #[sqlx(rename = "Role")]
    pub role: Option<serde_json::Value>,
    #[sqlx(rename = "Point")]
    pub point: Option<i32>,
    #[sqlx(rename = "SupporterInfoIndex")]
    pub supporter_info_index: Option<String>,
    #[sqlx(rename = "LastLoginDate")]
    pub last_login_date: Option<i64>,
    #[sqlx(rename = "UpdateDate")]
    pub update_date: Option<i64>,
    #[sqlx(rename = "Date")]
    pub date: Option<i64>,
}