use crate::models::game::map::map_active_info::MapActiveInfo;
use sqlx::SqlitePool;

/// Add a single MapActiveInfo record from a Rust struct.
pub async fn add_map_active_info(pool: &SqlitePool, data: &MapActiveInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MapActiveInfo (
    Uid,
    MapId,
    ActiveInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.map_id)
    .bind(&data.active_info)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_map_active_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MapActiveInfo>> {
    sqlx::query_as::<_, MapActiveInfo>("SELECT * FROM MapActiveInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MapActiveInfo rows for a UID.
pub async fn delete_map_active_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MapActiveInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MapActiveInfo> {
    sqlx::query_as::<_, MapActiveInfo>("SELECT * FROM MapActiveInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MapActiveInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MapActiveInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MapActiveInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MapActiveInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MapActiveInfo (
    Uid,
    MapId,
    ActiveInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.map_id)
    .bind(&data.active_info)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
