use sqlx::SqlitePool;

pub async fn has_claimed(pool: &SqlitePool, uid: i64, event_id: i32, reward_id: i32) -> sqlx::Result<bool> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM CharVoteRewardClaim WHERE Uid = ? AND EventId = ? AND RewardId = ?",
    )
    .bind(uid)
    .bind(event_id)
    .bind(reward_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn claim(pool: &SqlitePool, uid: i64, event_id: i32, reward_id: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT OR IGNORE INTO CharVoteRewardClaim (Uid, EventId, RewardId) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(event_id)
        .bind(reward_id)
        .execute(pool)
        .await?;
    Ok(())
}
