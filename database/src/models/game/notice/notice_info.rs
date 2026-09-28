use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct NoticeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "NoticeType")]
    pub notice_type: Option<i32>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "Thumbnail")]
    pub thumbnail: Option<String>,
    #[sqlx(rename = "StartTime")]
    pub start_time: Option<i64>,
    #[sqlx(rename = "EndTime")]
    pub end_time: Option<i64>,
    #[sqlx(rename = "WebUrl")]
    pub web_url: Option<String>,
    #[sqlx(rename = "NoticeContentsInfoIndex")]
    pub notice_contents_info_index: Option<String>,
    #[sqlx(rename = "PromotionBannerId")]
    pub promotion_banner_id: Option<i32>,
    #[sqlx(rename = "IsPin")]
    pub is_pin: Option<bool>,
    #[sqlx(rename = "Sort")]
    pub sort: Option<i32>,
    #[sqlx(rename = "SubType")]
    pub sub_type: Option<i32>,
    #[sqlx(rename = "IsInvert")]
    pub is_invert: Option<bool>,
}
