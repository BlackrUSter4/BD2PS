use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MailInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Type")]
    pub r#type: Option<i32>,
    #[sqlx(rename = "MailId")]
    pub mail_id: Option<i32>,
    #[sqlx(rename = "SenderText")]
    pub sender_text: Option<String>,
    #[sqlx(rename = "TitleText")]
    pub title_text: Option<String>,
    #[sqlx(rename = "MessageText")]
    pub message_text: Option<String>,
    #[sqlx(rename = "RewardExpireTime")]
    pub reward_expire_time: Option<i64>,
    #[sqlx(rename = "ItemType")]
    pub item_type: i32,
    #[sqlx(rename = "ItemId")]
    pub item_id: i32,
    #[sqlx(rename = "ItemCount")]
    pub item_count: i32,
    #[sqlx(rename = "IsOpen")]
    pub is_open: Option<bool>,
    #[sqlx(rename = "OpenTime")]
    pub open_time: Option<i64>,
    #[sqlx(rename = "CreateTime")]
    pub create_time: Option<i64>,
    #[sqlx(rename = "HistoryDeleteTime")]
    pub history_delete_time: Option<i64>,
    #[sqlx(rename = "IsCash")]
    pub is_cash: Option<bool>,
}
