use sqlx::SqlitePool;

pub async fn get_cleared(pool: &SqlitePool, uid: i64) -> Vec<i32> {
    sqlx::query_scalar::<_, i32>("SELECT Id FROM DailyStoryClearInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}

/// Returns true if this call newly cleared it (false if already cleared).
pub async fn try_clear(pool: &SqlitePool, uid: i64, id: i32) -> bool {
    let already = sqlx::query("SELECT 1 FROM DailyStoryClearInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .is_some();
    if already {
        return false;
    }
    sqlx::query("INSERT OR IGNORE INTO DailyStoryClearInfo (Uid, Id) VALUES (?, ?)")
        .bind(uid)
        .bind(id)
        .execute(pool)
        .await
        .is_ok()
}
