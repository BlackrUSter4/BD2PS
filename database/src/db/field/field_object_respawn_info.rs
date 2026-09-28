use crate::models::game::field::field_object_respawn_info::FieldObjectRespawnInfo;
use sqlx::SqlitePool;

/// Add a single FieldObjectRespawnInfo record from a Rust struct.
pub async fn add_field_object_respawn_info(
    pool: &SqlitePool,
    data: &FieldObjectRespawnInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldObjectRespawnInfo (
    Uid,
    FieldObjectGroupId,
    RespawnTime
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.field_object_group_id)
    .bind(&data.respawn_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_object_respawn_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<FieldObjectRespawnInfo>> {
    sqlx::query_as::<_, FieldObjectRespawnInfo>(
        "SELECT * FROM FieldObjectRespawnInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Get the respawn-tracking row for one field object group, if it's been saved.
pub async fn get_by_uid_and_group(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
) -> sqlx::Result<Option<FieldObjectRespawnInfo>> {
    sqlx::query_as::<_, FieldObjectRespawnInfo>(
        "SELECT * FROM FieldObjectRespawnInfo WHERE Uid = ? AND FieldObjectGroupId = ?",
    )
    .bind(uid)
    .bind(group_id)
    .fetch_optional(pool)
    .await
}

/// Save/overwrite the respawn timer for one group.
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    group_id: i32,
    respawn_time: i64,
) -> sqlx::Result<()> {
    if get_by_uid_and_group(pool, uid, group_id).await?.is_some() {
        sqlx::query(
            "UPDATE FieldObjectRespawnInfo SET RespawnTime = ? WHERE Uid = ? AND FieldObjectGroupId = ?",
        )
        .bind(respawn_time)
        .bind(uid)
        .bind(group_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO FieldObjectRespawnInfo (Uid, FieldObjectGroupId, RespawnTime) VALUES (?, ?, ?)",
        )
        .bind(uid)
        .bind(group_id)
        .bind(respawn_time)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Delete all FieldObjectRespawnInfo rows for a UID.
pub async fn delete_field_object_respawn_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldObjectRespawnInfo WHERE Uid = ?")
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
) -> sqlx::Result<FieldObjectRespawnInfo> {
    sqlx::query_as::<_, FieldObjectRespawnInfo>(
        "SELECT * FROM FieldObjectRespawnInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<FieldObjectRespawnInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldObjectRespawnInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldObjectRespawnInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldObjectRespawnInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldObjectRespawnInfo (
    Uid,
    FieldObjectGroupId,
    RespawnTime
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.field_object_group_id)
    .bind(&data.respawn_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
