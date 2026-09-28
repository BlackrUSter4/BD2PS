use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PersonalInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "Title")]
    pub title: Option<String>,
    #[sqlx(rename = "Url")]
    pub url: Option<String>,
    #[sqlx(rename = "StartDate")]
    pub start_date: Option<i64>,
    #[sqlx(rename = "EndDate")]
    pub end_date: Option<i64>,
}