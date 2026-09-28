use sqlx::SqlitePool;

/// Attempts to record a special episode as cleared. Returns true if this call newly inserted it
/// (i.e. the reward should be granted), false if it was already cleared before (no re-grant).
pub async fn try_clear(pool: &SqlitePool, uid: i64, group_id: i32, id: i32) -> bool {
    let result = sqlx::query(
        "INSERT INTO FriendshipSpecialEpisodeClear (Uid, GroupId, Id) VALUES (?, ?, ?) \
         ON CONFLICT(Uid, GroupId, Id) DO NOTHING",
    )
    .bind(uid)
    .bind(group_id)
    .bind(id)
    .execute(pool)
    .await;

    matches!(result, Ok(r) if r.rows_affected() > 0)
}

pub async fn get_all(pool: &SqlitePool, uid: i64) -> Vec<(i32, i32)> {
    sqlx::query_as::<_, (i32, i32)>("SELECT GroupId, Id FROM FriendshipSpecialEpisodeClear WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}
