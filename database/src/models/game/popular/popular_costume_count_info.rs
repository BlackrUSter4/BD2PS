use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct PopularCostumeCountInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "CountType0")]
    pub count_type_0: Option<i64>,
    #[sqlx(rename = "CountType1")]
    pub count_type_1: Option<i64>,
    #[sqlx(rename = "CountType2")]
    pub count_type_2: Option<i64>,
}