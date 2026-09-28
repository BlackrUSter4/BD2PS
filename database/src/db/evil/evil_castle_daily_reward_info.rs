use crate::models::game::evil::evil_castle_daily_reward_info::EvilCastleDailyRewardInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<EvilCastleDailyRewardInfo>> {
    sqlx::query_as::<_, EvilCastleDailyRewardInfo>(
        "SELECT * FROM EvilCastleDailyRewardInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// Record today's claim and advance the streak counter (upsert on the
/// UNIQUE Uid).
pub async fn set_claimed(pool: &SqlitePool, uid: i64, date: &str, claim_count: i32) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO EvilCastleDailyRewardInfo (Uid, LastClaimDate, ClaimCount) VALUES (?, ?, ?)
         ON CONFLICT(Uid) DO UPDATE SET LastClaimDate = excluded.LastClaimDate, ClaimCount = excluded.ClaimCount",
    )
    .bind(uid)
    .bind(date)
    .bind(claim_count)
    .execute(pool)
    .await?;
    Ok(())
}
