use crate::models::game::field::field_trap_info::FieldTrapInfo;
use sqlx::SqlitePool;

/// Add a single FieldTrapInfo record from a Rust struct.
pub async fn add_field_trap_info(pool: &SqlitePool, data: &FieldTrapInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FieldTrapInfo (
    Uid,
    PackId,
    MapId,
    TrapId,
    State
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
    .bind(&data.pack_id)
    .bind(&data.map_id)
    .bind(&data.trap_id)
    .bind(&data.state)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_field_trap_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FieldTrapInfo>> {
    sqlx::query_as::<_, FieldTrapInfo>("SELECT * FROM FieldTrapInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all FieldTrapInfo rows for a UID.
pub async fn delete_field_trap_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FieldTrapInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<FieldTrapInfo> {
    sqlx::query_as::<_, FieldTrapInfo>("SELECT * FROM FieldTrapInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<FieldTrapInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FieldTrapInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FieldTrapInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FieldTrapInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FieldTrapInfo (
    Uid,
    PackId,
    MapId,
    TrapId,
    State
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
    .bind(&data.pack_id)
    .bind(&data.map_id)
    .bind(&data.trap_id)
    .bind(&data.state)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
