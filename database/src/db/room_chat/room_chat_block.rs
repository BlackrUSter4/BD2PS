use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct RoomChatBlock {
    pub uid: i64,
    pub target_owner_index: i64,
    pub target_user_id: Option<String>,
    pub block_date: i64,
}

/// Resolves an OwnerIndex to the account's real Uid, if one exists.
pub async fn resolve_uid(pool: &SqlitePool, owner_index: i64) -> Option<i64> {
    sqlx::query_as::<_, (i64,)>("SELECT Uid FROM UserInfo WHERE OwnerIndex = ?")
        .bind(owner_index)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .map(|(uid,)| uid)
}

/// Resolves an OwnerIndex to the account's real UserId display string, if one exists.
pub async fn resolve_user_id(pool: &SqlitePool, owner_index: i64) -> Option<String> {
    sqlx::query_as::<_, (Option<String>,)>("SELECT UserId FROM UserInfo WHERE OwnerIndex = ?")
        .bind(owner_index)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .and_then(|(user_id,)| user_id)
}

pub async fn add(pool: &SqlitePool, uid: i64, target_owner_index: i64, target_user_id: Option<&str>, block_date: i64) {
    let _ = sqlx::query(
        "INSERT INTO RoomChatBlock (Uid, TargetOwnerIndex, TargetUserId, BlockDate) VALUES (?, ?, ?, ?) \
         ON CONFLICT(Uid, TargetOwnerIndex) DO UPDATE SET TargetUserId = excluded.TargetUserId, BlockDate = excluded.BlockDate",
    )
    .bind(uid)
    .bind(target_owner_index)
    .bind(target_user_id)
    .bind(block_date)
    .execute(pool)
    .await;
}

pub async fn remove(pool: &SqlitePool, uid: i64, target_owner_index: i64) {
    let _ = sqlx::query("DELETE FROM RoomChatBlock WHERE Uid = ? AND TargetOwnerIndex = ?")
        .bind(uid)
        .bind(target_owner_index)
        .execute(pool)
        .await;
}

pub async fn get_all(pool: &SqlitePool, uid: i64) -> Vec<RoomChatBlock> {
    sqlx::query_as::<_, RoomChatBlock>(
        "SELECT Uid as uid, TargetOwnerIndex as target_owner_index, TargetUserId as target_user_id, BlockDate as block_date \
         FROM RoomChatBlock WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}
