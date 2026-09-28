use crate::models::game::action::action_matching_client_info::ActionMatchingClientInfo;
use sqlx::SqlitePool;

/// Add a single ActionMatchingClientInfo record from a Rust struct.
pub async fn add_action_matching_client_info(
    pool: &SqlitePool,
    data: &ActionMatchingClientInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ActionMatchingClientInfo (
    Uid,
    UserInfo,
    IsRoomMaster,
    EnterTime,
    Guid,
    ActionChar
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.user_info_index)
    .bind(&data.is_room_master)
    .bind(&data.enter_time)
    .bind(&data.guid)
    .bind(&data.action_char)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_action_matching_client_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ActionMatchingClientInfo>> {
    sqlx::query_as::<_, ActionMatchingClientInfo>(
        "SELECT * FROM ActionMatchingClientInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all ActionMatchingClientInfo rows for a UID.
pub async fn delete_action_matching_client_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ActionMatchingClientInfo WHERE Uid = ?")
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
) -> sqlx::Result<ActionMatchingClientInfo> {
    sqlx::query_as::<_, ActionMatchingClientInfo>(
        "SELECT * FROM ActionMatchingClientInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ActionMatchingClientInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ActionMatchingClientInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ActionMatchingClientInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ActionMatchingClientInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ActionMatchingClientInfo (
    Uid,
    UserInfo,
    IsRoomMaster,
    EnterTime,
    Guid,
    ActionChar
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.user_info_index)
    .bind(&data.is_room_master)
    .bind(&data.enter_time)
    .bind(&data.guid)
    .bind(&data.action_char)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
