use crate::models::game::env::env_info::EnvInfo;
use sqlx::SqlitePool;

/// Add a single EnvInfo record from a Rust struct.
pub async fn add_env_info(pool: &SqlitePool, data: &EnvInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EnvInfo (
    Uid,
    IsLive,
    UseSdk
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.is_live)
    .bind(&data.use_sdk)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_env_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EnvInfo>> {
    sqlx::query_as::<_, EnvInfo>("SELECT * FROM EnvInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EnvInfo rows for a UID.
pub async fn delete_env_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EnvInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<EnvInfo> {
    sqlx::query_as::<_, EnvInfo>("SELECT * FROM EnvInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<EnvInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EnvInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EnvInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EnvInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EnvInfo (
    Uid,
    IsLive,
    UseSdk
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.is_live)
    .bind(&data.use_sdk)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
