use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharVoteFavorite {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "CandidateId")]
    pub candidate_id: i32,
}
