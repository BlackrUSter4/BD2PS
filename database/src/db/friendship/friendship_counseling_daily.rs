use crate::models::game::friendship::friendship_counseling_daily::FriendshipCounselingDaily;
use sqlx::SqlitePool;

/// Real values from `FriendshipDefaultTable`: 3 total counseling sessions/day across all
/// costumes, 1 per individual costume/day.
pub const MAX_TOTAL_PER_DAY: i32 = 3;
pub const MAX_PER_COSTUME_PER_DAY: i32 = 1;

pub fn today() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

async fn get_or_default(pool: &SqlitePool, uid: i64, day: &str) -> FriendshipCounselingDaily {
    sqlx::query_as::<_, FriendshipCounselingDaily>("SELECT * FROM FriendshipCounselingDaily WHERE Uid = ? AND Day = ?")
        .bind(uid)
        .bind(day)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(FriendshipCounselingDaily {
            uid,
            day: day.to_string(),
            total_count: 0,
            complete_reward_granted: 0,
        })
}

async fn costume_already_done_today(pool: &SqlitePool, uid: i64, costume_id: i32, day: &str) -> bool {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM FriendshipCounselingCostumeDaily WHERE Uid = ? AND CostumeId = ? AND Day = ?",
    )
    .bind(uid)
    .bind(costume_id)
    .bind(day)
    .fetch_one(pool)
    .await
    .unwrap_or(0)
        > 0
}

/// Returns `(allowed, should_grant_completion_bonus)`. If `allowed`, the caller should proceed
/// to award counseling exp and then call [`record_session`] to persist the AP spend.
pub async fn check_and_would_allow(pool: &SqlitePool, uid: i64, costume_id: i32) -> bool {
    let day = today();
    if costume_already_done_today(pool, uid, costume_id, &day).await {
        return false;
    }
    let row = get_or_default(pool, uid, &day).await;
    row.total_count < MAX_TOTAL_PER_DAY
}

/// Persists that a counseling AP was spent on `costume_id` today. Returns true if this spend
/// just reached the daily total cap AND the completion bonus hasn't been granted yet today
/// (caller should grant `counselingCompleteRewardType`/`counselingCompleteRewardCount` and then
/// call [`mark_completion_reward_granted`]).
pub async fn record_session(pool: &SqlitePool, uid: i64, costume_id: i32) -> bool {
    let day = today();
    let _ = sqlx::query(
        "INSERT INTO FriendshipCounselingCostumeDaily (Uid, CostumeId, Day) VALUES (?, ?, ?) \
         ON CONFLICT(Uid, CostumeId, Day) DO NOTHING",
    )
    .bind(uid)
    .bind(costume_id)
    .bind(&day)
    .execute(pool)
    .await;

    let mut row = get_or_default(pool, uid, &day).await;
    row.total_count += 1;
    let _ = sqlx::query(
        "INSERT INTO FriendshipCounselingDaily (Uid, Day, TotalCount, CompleteRewardGranted) VALUES (?, ?, ?, ?) \
         ON CONFLICT(Uid, Day) DO UPDATE SET TotalCount = excluded.TotalCount",
    )
    .bind(uid)
    .bind(&day)
    .bind(row.total_count)
    .bind(row.complete_reward_granted)
    .execute(pool)
    .await;

    row.total_count >= MAX_TOTAL_PER_DAY && row.complete_reward_granted == 0
}

pub async fn mark_completion_reward_granted(pool: &SqlitePool, uid: i64) {
    let day = today();
    let _ = sqlx::query("UPDATE FriendshipCounselingDaily SET CompleteRewardGranted = 1 WHERE Uid = ? AND Day = ?")
        .bind(uid)
        .bind(&day)
        .execute(pool)
        .await;
}
