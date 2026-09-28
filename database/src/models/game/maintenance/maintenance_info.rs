use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MaintenanceInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "MarketType")]
    pub market_type: Option<i32>,
    #[sqlx(rename = "Version")]
    pub version: Option<String>,
    #[sqlx(rename = "BundleVersion")]
    pub bundle_version: Option<String>,
    #[sqlx(rename = "IsBundleUpdate")]
    pub is_bundle_update: Option<bool>,
    #[sqlx(rename = "MaintenanceType")]
    pub maintenance_type: Option<i32>,
    #[sqlx(rename = "Date")]
    pub date: Option<String>,
    #[sqlx(rename = "RegionList")]
    pub region_list: Option<String>,
    #[sqlx(rename = "UseDsa")]
    pub use_dsa: Option<i32>,
    #[sqlx(rename = "MaintenanceUrl")]
    pub maintenance_url: Option<String>,
    #[sqlx(rename = "UseMaintenanceUrl")]
    pub use_maintenance_url: Option<i32>,
    #[sqlx(rename = "DownloadUrl")]
    pub download_url: Option<String>,
    #[sqlx(rename = "Notice")]
    pub notice: Option<String>,
    #[sqlx(rename = "BundleVersionSd")]
    pub bundle_version_sd: Option<String>,
}