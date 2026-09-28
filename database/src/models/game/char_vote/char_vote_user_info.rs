use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharVoteUserInfo {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "EventId")]
    pub event_id: i32,
    #[sqlx(rename = "Round")]
    pub round: i32,
    #[sqlx(rename = "CandidateId")]
    pub candidate_id: i32,
    #[sqlx(rename = "TotalCount")]
    pub total_count: i32,
    #[sqlx(rename = "NormalCount")]
    pub normal_count: i32,
    #[sqlx(rename = "AdditionalCount")]
    pub additional_count: i32,
}
