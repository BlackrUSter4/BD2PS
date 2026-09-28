use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CostumeInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "InvenIndex")]
    pub inven_index: Option<i64>,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Level")]
    pub level: Option<i32>,
    #[sqlx(rename = "UseChar")]
    pub use_char: Option<i64>,
    #[sqlx(rename = "PictorialbookInfoIndex")]
    pub pictorialbook_info_index: Option<String>,
    #[sqlx(rename = "SortId")]
    pub sort_id: Option<i32>,
    #[sqlx(rename = "UseMyRoomCount")]
    pub use_my_room_count: Option<i32>,
    #[sqlx(rename = "PotentialId")]
    pub potential_id: Option<String>,
    #[sqlx(rename = "DesignId")]
    pub design_id: Option<i32>,
}
