use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct MiniGameRhythmPlayInfo {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "Id")]
    pub id: Option<i32>,
    #[sqlx(rename = "ModeType")]
    pub mode_type: Option<i32>,
    #[sqlx(rename = "BestRecordValue")]
    pub best_record_value: Option<i32>,
    #[sqlx(rename = "BestGradeType")]
    pub best_grade_type: Option<i32>,
    #[sqlx(rename = "BestComboType")]
    pub best_combo_type: Option<i32>,
}