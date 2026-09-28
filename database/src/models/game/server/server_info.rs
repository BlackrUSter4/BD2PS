use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct ServerInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Region")]
    pub region: Option<i32>,
    #[sqlx(rename = "GameServerInfo")]
    pub game_server_info: Option<String>,
    #[sqlx(rename = "CdnInfo")]
    pub cdn_info: Option<String>,
    #[sqlx(rename = "OpenFlag")]
    pub open_flag: Option<i32>,
    #[sqlx(rename = "LogServerInfo")]
    pub log_server_info: Option<String>,
    #[sqlx(rename = "CharServerInfo")]
    pub char_server_info: Option<String>,
    #[sqlx(rename = "CouponWebInfo")]
    pub coupon_web_info: Option<String>,
    #[sqlx(rename = "GameDataInfo")]
    pub game_data_info: Option<String>,
    #[sqlx(rename = "GameDataVersion")]
    pub game_data_version: Option<String>,
}