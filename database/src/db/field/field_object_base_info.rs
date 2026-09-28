use crate::models::game::field::field_object_base_info::FieldObjectBaseInfo;
use sqlx::SqlitePool;

/// Add a single FieldObjectBaseInfo record from a Rust struct.
pub async fn add_field_object_base_info(
    pool: &SqlitePool,
    data: &FieldObjectBaseInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldObjectBaseInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_object_base_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<FieldObjectBaseInfo>> {
    sqlx::query_as::<_, FieldObjectBaseInfo>("SELECT * FROM FieldObjectBaseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all FieldObjectBaseInfo rows for a UID.
pub async fn delete_field_object_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldObjectBaseInfo WHERE Uid = ?")
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
) -> sqlx::Result<FieldObjectBaseInfo> {
    sqlx::query_as::<_, FieldObjectBaseInfo>(
        "SELECT * FROM FieldObjectBaseInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<FieldObjectBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldObjectBaseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldObjectBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldObjectBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldObjectBaseInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
