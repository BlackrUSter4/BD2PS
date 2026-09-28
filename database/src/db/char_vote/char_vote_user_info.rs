use crate::models::game::char_vote::char_vote_user_info::CharVoteUserInfo;
use sqlx::SqlitePool;

pub async fn get(
    pool: &SqlitePool,
    uid: i64,
    event_id: i32,
    round: i32,
    candidate_id: i32,
) -> sqlx::Result<Option<CharVoteUserInfo>> {
    sqlx::query_as::<_, CharVoteUserInfo>(
        "SELECT * FROM CharVoteUserInfo WHERE Uid = ? AND EventId = ? AND Round = ? AND CandidateId = ?",
    )
    .bind(uid)
    .bind(event_id)
    .bind(round)
    .bind(candidate_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_by_round(
    pool: &SqlitePool,
    uid: i64,
    event_id: i32,
    round: i32,
) -> sqlx::Result<Vec<CharVoteUserInfo>> {
    sqlx::query_as::<_, CharVoteUserInfo>(
        "SELECT * FROM CharVoteUserInfo WHERE Uid = ? AND EventId = ? AND Round = ?",
    )
    .bind(uid)
    .bind(event_id)
    .bind(round)
    .fetch_all(pool)
    .await
}

/// Adds to an existing (uid, event, round, candidate) row's counts, creating it if absent.
pub async fn add_vote(
    pool: &SqlitePool,
    uid: i64,
    event_id: i32,
    round: i32,
    candidate_id: i32,
    normal_delta: i32,
    additional_delta: i32,
) -> sqlx::Result<CharVoteUserInfo> {
    sqlx::query(
        r#"
INSERT INTO CharVoteUserInfo (Uid, EventId, Round, CandidateId, TotalCount, NormalCount, AdditionalCount)
VALUES (?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid, EventId, Round, CandidateId) DO UPDATE SET
    TotalCount = TotalCount + excluded.TotalCount,
    NormalCount = NormalCount + excluded.NormalCount,
    AdditionalCount = AdditionalCount + excluded.AdditionalCount
"#,
    )
    .bind(uid)
    .bind(event_id)
    .bind(round)
    .bind(candidate_id)
    .bind(normal_delta + additional_delta)
    .bind(normal_delta)
    .bind(additional_delta)
    .execute(pool)
    .await?;

    Ok(get(pool, uid, event_id, round, candidate_id).await?.unwrap_or_default())
}

/// Real-account cross-account totals per candidate for one round, highest first.
pub async fn total_votes_by_candidate(
    pool: &SqlitePool,
    event_id: i32,
    round: i32,
) -> sqlx::Result<Vec<(i32, i64)>> {
    sqlx::query_as::<_, (i32, i64)>(
        r#"
SELECT CandidateId, SUM(TotalCount) FROM CharVoteUserInfo
WHERE EventId = ? AND Round = ?
GROUP BY CandidateId
ORDER BY SUM(TotalCount) DESC
"#,
    )
    .bind(event_id)
    .bind(round)
    .fetch_all(pool)
    .await
}

/// Real-account cross-account totals per candidate across every round of an event, highest first.
pub async fn total_votes_across_rounds(pool: &SqlitePool, event_id: i32) -> sqlx::Result<Vec<(i32, i64)>> {
    sqlx::query_as::<_, (i32, i64)>(
        r#"
SELECT CandidateId, SUM(TotalCount) FROM CharVoteUserInfo
WHERE EventId = ?
GROUP BY CandidateId
ORDER BY SUM(TotalCount) DESC
"#,
    )
    .bind(event_id)
    .fetch_all(pool)
    .await
}

/// One account's own all-round total for an event (used for milestone-reward checks).
pub async fn account_total_votes(pool: &SqlitePool, uid: i64, event_id: i32) -> sqlx::Result<i64> {
    let row: (Option<i64>,) =
        sqlx::query_as("SELECT SUM(TotalCount) FROM CharVoteUserInfo WHERE Uid = ? AND EventId = ?")
            .bind(uid)
            .bind(event_id)
            .fetch_one(pool)
            .await?;
    Ok(row.0.unwrap_or(0))
}
