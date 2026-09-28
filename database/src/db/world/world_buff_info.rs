use crate::models::game::world::world_buff_info::WorldBuffInfo;
use sqlx::SqlitePool;

/// Add a single WorldBuffInfo record from a Rust struct.
pub async fn add_world_buff_info(pool: &SqlitePool, data: &WorldBuffInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO WorldBuffInfo (
    Uid,
    Id,
    RemainingTime
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.remaining_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_world_buff_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<WorldBuffInfo>> {
    sqlx::query_as::<_, WorldBuffInfo>("SELECT * FROM WorldBuffInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all WorldBuffInfo rows for a UID.
pub async fn delete_world_buff_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM WorldBuffInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<WorldBuffInfo> {
    sqlx::query_as::<_, WorldBuffInfo>("SELECT * FROM WorldBuffInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<WorldBuffInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM WorldBuffInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, WorldBuffInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &WorldBuffInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO WorldBuffInfo (
    Uid,
    Id,
    RemainingTime
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.remaining_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
