use sqlx::SqlitePool;

pub async fn is_open(pool: &SqlitePool, uid: i64, r#type: i32) -> bool {
    sqlx::query("SELECT 1 FROM ContentOpenInfo WHERE Uid = ? AND Type = ?")
        .bind(uid)
        .bind(r#type)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .is_some()
}

/// Returns true if this call newly opened it (false if already open).
pub async fn try_open(pool: &SqlitePool, uid: i64, r#type: i32) -> bool {
    if is_open(pool, uid, r#type).await {
        return false;
    }
    sqlx::query("INSERT OR IGNORE INTO ContentOpenInfo (Uid, Type) VALUES (?, ?)")
        .bind(uid)
        .bind(r#type)
        .execute(pool)
        .await
        .is_ok()
}
