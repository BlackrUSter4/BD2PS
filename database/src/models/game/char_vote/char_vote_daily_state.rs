use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// `normal_vote_candidate_ids` is a comma-separated list of candidate ids the account has cast
/// a free daily ("normal") vote for since the last reset — small, simple, avoids a whole extra
/// join table for what's just a short per-day list.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "PascalCase")]
#[sqlx(rename_all = "PascalCase")]
pub struct CharVoteDailyState {
    #[sqlx(rename = "Uid")]
    pub uid: i64,
    #[sqlx(rename = "LastResetDate")]
    pub last_reset_date: String,
    #[sqlx(rename = "NormalVoteCandidateIds")]
    pub normal_vote_candidate_ids: String,
}

impl Default for CharVoteDailyState {
    fn default() -> Self {
        Self { uid: 0, last_reset_date: String::new(), normal_vote_candidate_ids: String::new() }
    }
}

impl CharVoteDailyState {
    pub fn candidate_ids(&self) -> Vec<i32> {
        self.normal_vote_candidate_ids
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect()
    }
}
