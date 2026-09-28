use sqlx::SqlitePool;

/// Records a counseling session as permanently completed for this costume (used to populate
/// `CounselingDBInfo.counseling_session_id` in `FriendshipInfoResponse`). Idempotent.
pub async fn mark_done(pool: &SqlitePool, uid: i64, costume_id: i32, session_id: i32) {
    let _ = sqlx::query(
        "INSERT INTO FriendshipCounselingSession (Uid, CostumeId, SessionId) VALUES (?, ?, ?) \
         ON CONFLICT(Uid, CostumeId, SessionId) DO NOTHING",
    )
    .bind(uid)
    .bind(costume_id)
    .bind(session_id)
    .execute(pool)
    .await;
}

/// Returns (costume_id, session_id) pairs for every session ever completed by this account,
/// across all costumes.
pub async fn get_all(pool: &SqlitePool, uid: i64) -> Vec<(i32, i32)> {
    sqlx::query_as::<_, (i32, i32)>("SELECT CostumeId, SessionId FROM FriendshipCounselingSession WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
}
