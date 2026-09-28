use crate::models::game::char_vote::char_vote_daily_state::CharVoteDailyState;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<CharVoteDailyState>> {
    sqlx::query_as::<_, CharVoteDailyState>("SELECT * FROM CharVoteDailyState WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn insert(pool: &SqlitePool, data: &CharVoteDailyState) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO CharVoteDailyState (Uid, LastResetDate, NormalVoteCandidateIds) VALUES (?, ?, ?)")
        .bind(data.uid)
        .bind(&data.last_reset_date)
        .bind(&data.normal_vote_candidate_ids)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<CharVoteDailyState> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = CharVoteDailyState { uid, ..Default::default() };
    insert(pool, &data).await?;
    Ok(data)
}

/// Resets the daily normal-vote list if `today` differs from the stored last-reset date.
/// Returns the (possibly just-reset) state.
pub async fn reset_if_new_day(pool: &SqlitePool, uid: i64, today: &str) -> sqlx::Result<CharVoteDailyState> {
    let state = get_or_create(pool, uid).await?;
    if state.last_reset_date != today {
        sqlx::query("UPDATE CharVoteDailyState SET LastResetDate = ?, NormalVoteCandidateIds = '' WHERE Uid = ?")
            .bind(today)
            .bind(uid)
            .execute(pool)
            .await?;
        return Ok(CharVoteDailyState {
            last_reset_date: today.to_string(),
            normal_vote_candidate_ids: String::new(),
            ..state
        });
    }
    Ok(state)
}

pub async fn add_normal_vote_candidate(
    pool: &SqlitePool,
    uid: i64,
    today: &str,
    candidate_id: i32,
) -> sqlx::Result<()> {
    let state = reset_if_new_day(pool, uid, today).await?;
    let mut ids = state.candidate_ids();
    if !ids.contains(&candidate_id) {
        ids.push(candidate_id);
    }
    let joined = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    sqlx::query("UPDATE CharVoteDailyState SET NormalVoteCandidateIds = ? WHERE Uid = ?")
        .bind(joined)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
