use crate::models::game::room::room_client_info::RoomClientInfo;
use sqlx::SqlitePool;

/// Add a single RoomClientInfo record from a Rust struct.
pub async fn add_room_client_info(pool: &SqlitePool, data: &RoomClientInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RoomClientInfo (
    Uid,
    OwnerIndex,
    Guid,
    EnterTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.guid)
    .bind(&data.enter_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_room_client_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<RoomClientInfo>> {
    sqlx::query_as::<_, RoomClientInfo>("SELECT * FROM RoomClientInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RoomClientInfo rows for a UID.
pub async fn delete_room_client_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RoomClientInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<RoomClientInfo> {
    sqlx::query_as::<_, RoomClientInfo>("SELECT * FROM RoomClientInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<RoomClientInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RoomClientInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RoomClientInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RoomClientInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RoomClientInfo (
    Uid,
    OwnerIndex,
    Guid,
    EnterTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.guid)
    .bind(&data.enter_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
