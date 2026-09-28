use crate::models::game::optimize::optimize_base_info::OptimizeBaseInfo;
use sqlx::SqlitePool;

/// Add a single OptimizeBaseInfo record from a Rust struct.
pub async fn add_optimize_base_info(
    pool: &SqlitePool,
    data: &OptimizeBaseInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO OptimizeBaseInfo (
    Uid,
    OptimizeIndex,
    OptimizeValue,
    OptimizeProperty
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.optimize_index)
    .bind(&data.optimize_value)
    .bind(&data.optimize_property)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_optimize_base_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<OptimizeBaseInfo>> {
    sqlx::query_as::<_, OptimizeBaseInfo>("SELECT * FROM OptimizeBaseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all OptimizeBaseInfo rows for a UID.
pub async fn delete_optimize_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM OptimizeBaseInfo WHERE Uid = ?")
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
) -> sqlx::Result<OptimizeBaseInfo> {
    sqlx::query_as::<_, OptimizeBaseInfo>(
        "SELECT * FROM OptimizeBaseInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<OptimizeBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM OptimizeBaseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, OptimizeBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &OptimizeBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO OptimizeBaseInfo (
    Uid,
    OptimizeIndex,
    OptimizeValue,
    OptimizeProperty
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.optimize_index)
    .bind(&data.optimize_value)
    .bind(&data.optimize_property)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
