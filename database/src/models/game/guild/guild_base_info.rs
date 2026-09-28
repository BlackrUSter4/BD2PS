use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct GuildBaseInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i64>,
    #[sqlx(rename = "Name")]
    pub name: Option<String>,
    #[sqlx(rename = "Icon")]
    pub icon: Option<i32>,
    #[sqlx(rename = "IconColor")]
    pub icon_color: Option<String>,
    #[sqlx(rename = "Grade")]
    pub grade: Option<i32>,
}