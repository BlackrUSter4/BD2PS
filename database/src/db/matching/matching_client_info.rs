use crate::models::game::matching::matching_client_info::MatchingClientInfo;
use sqlx::SqlitePool;

/// Add a single MatchingClientInfo record from a Rust struct.
pub async fn add_matching_client_info(
    pool: &SqlitePool,
    data: &MatchingClientInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MatchingClientInfo (
    Uid,
    UserInfo,
    IsRoomMaster,
    EnterTime,
    Guid
) VALUES (
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
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_matching_client_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MatchingClientInfo>> {
    sqlx::query_as::<_, MatchingClientInfo>("SELECT * FROM MatchingClientInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MatchingClientInfo rows for a UID.
pub async fn delete_matching_client_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MatchingClientInfo WHERE Uid = ?")
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
) -> sqlx::Result<MatchingClientInfo> {
    sqlx::query_as::<_, MatchingClientInfo>(
        "SELECT * FROM MatchingClientInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MatchingClientInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MatchingClientInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MatchingClientInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MatchingClientInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MatchingClientInfo (
    Uid,
    UserInfo,
    IsRoomMaster,
    EnterTime,
    Guid
) VALUES (
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
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
