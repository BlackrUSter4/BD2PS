use crate::models::game::field::field_object_info::FieldObjectInfo;
use sqlx::SqlitePool;

/// Add a single FieldObjectInfo record from a Rust struct.
pub async fn add_field_object_info(pool: &SqlitePool, data: &FieldObjectInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldObjectInfo (
    Uid,
    Id,
    PositionIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.position_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_object_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<FieldObjectInfo>> {
    sqlx::query_as::<_, FieldObjectInfo>("SELECT * FROM FieldObjectInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for one field object id, if it's already been saved.
pub async fn get_by_uid_and_id(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
) -> sqlx::Result<Option<FieldObjectInfo>> {
    sqlx::query_as::<_, FieldObjectInfo>("SELECT * FROM FieldObjectInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Save/overwrite the position link for one field object id.
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
    position_index: i64,
) -> sqlx::Result<()> {
    if get_by_uid_and_id(pool, uid, id).await?.is_some() {
        sqlx::query("UPDATE FieldObjectInfo SET PositionIndex = ? WHERE Uid = ? AND Id = ?")
            .bind(position_index)
            .bind(uid)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO FieldObjectInfo (Uid, Id, PositionIndex) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(id)
            .bind(position_index)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Delete all FieldObjectInfo rows for a UID.
pub async fn delete_field_object_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldObjectInfo WHERE Uid = ?")
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
) -> sqlx::Result<FieldObjectInfo> {
    sqlx::query_as::<_, FieldObjectInfo>(
        "SELECT * FROM FieldObjectInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<FieldObjectInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldObjectInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldObjectInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldObjectInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldObjectInfo (
    Uid,
    Id,
    PositionIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.position_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
