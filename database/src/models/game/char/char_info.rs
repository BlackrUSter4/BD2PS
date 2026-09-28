use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Hp")]
    pub hp: Option<i64>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "CostumeId")]
    pub costume_id: Option<i32>,
    #[sqlx(rename = "Exp")]
    pub exp: Option<i32>,
    #[sqlx(rename = "UseCostume")]
    pub use_costume: Option<i64>,
    #[sqlx(rename = "TalentLevel")]
    pub talent_level: Option<i32>,
    #[sqlx(rename = "TalentExp")]
    pub talent_exp: Option<i32>,
    #[sqlx(rename = "SolidarityReward")]
    pub solidarity_reward: Option<i32>,
    #[sqlx(rename = "ExpiryTime")]
    pub expiry_time: Option<i64>,
    #[sqlx(rename = "PictorialbookInfoIndex")]
    pub pictorialbook_info_index: Option<String>,
    #[sqlx(rename = "ConnectPotentialCostume")]
    pub connect_potential_costume: Option<i32>,
    #[sqlx(rename = "ClassLevel")]
    pub class_level: i32,
}
