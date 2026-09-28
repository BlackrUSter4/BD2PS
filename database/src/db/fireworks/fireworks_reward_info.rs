use sqlx::SqlitePool;

pub async fn get_rewarded_groups(pool: &SqlitePool, uid: i64) -> Vec<i32> {
    sqlx::query_scalar::<_, i32>("SELECT GroupId FROM FireWorksRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

/// Returns true if this call newly claimed it (false if already claimed).
pub async fn try_claim(pool: &SqlitePool, uid: i64, event_schedule_id: i32, group_id: i32) -> bool {
    let already = sqlx::query(
        "SELECT 1 FROM FireWorksRewardInfo WHERE Uid = ? AND EventScheduleId = ? AND GroupId = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(group_id)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .is_some();
    if already {
        return false;
    }
    sqlx::query("INSERT OR IGNORE INTO FireWorksRewardInfo (Uid, EventScheduleId, GroupId) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(event_schedule_id)
        .bind(group_id)
        .execute(pool)
        .await
        .is_ok()
}
