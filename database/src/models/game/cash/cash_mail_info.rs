use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CashMailInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MailInfoIndex")]
    pub mail_info_index: Option<String>,
    #[sqlx(rename = "TotalCount")]
    pub total_count: Option<i32>,
    #[sqlx(rename = "MaxInvenIndex")]
    pub max_inven_index: Option<i64>,
}