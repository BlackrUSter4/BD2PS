use crate::models::game::action::action_relay_user_info::ActionRelayUserInfo;
use sqlx::SqlitePool;

/// Add a single ActionRelayUserInfo record from a Rust struct.
pub async fn add_action_relay_user_info(
    pool: &SqlitePool,
    data: &ActionRelayUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ActionRelayUserInfo (
    Uid,
    ActionUserInfo,
    PlayerSyncInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.action_user_info_index)
    .bind(&data.player_sync_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_action_relay_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ActionRelayUserInfo>> {
    sqlx::query_as::<_, ActionRelayUserInfo>("SELECT * FROM ActionRelayUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ActionRelayUserInfo rows for a UID.
pub async fn delete_action_relay_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ActionRelayUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<ActionRelayUserInfo> {
    sqlx::query_as::<_, ActionRelayUserInfo>(
        "SELECT * FROM ActionRelayUserInfo WHERE Uid = ? AND Index = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<ActionRelayUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ActionRelayUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ActionRelayUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ActionRelayUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ActionRelayUserInfo (
    Uid,
    ActionUserInfo,
    PlayerSyncInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.action_user_info_index)
    .bind(&data.player_sync_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
