use crate::models::game::knockback::knockback_info::KnockbackInfo;
use sqlx::SqlitePool;

/// Add a single KnockbackInfo record from a Rust struct.
pub async fn add_knockback_info(pool: &SqlitePool, data: &KnockbackInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO KnockbackInfo (
    Uid,
    KnockbackDir,
    KnockbackValue,
    KnockbackSpeed,
    IsGuard
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
    .bind(&data.knockback_dir_index)
    .bind(&data.knockback_value)
    .bind(&data.knockback_speed)
    .bind(&data.is_guard)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_knockback_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<KnockbackInfo>> {
    sqlx::query_as::<_, KnockbackInfo>("SELECT * FROM KnockbackInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all KnockbackInfo rows for a UID.
pub async fn delete_knockback_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM KnockbackInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<KnockbackInfo> {
    sqlx::query_as::<_, KnockbackInfo>("SELECT * FROM KnockbackInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<KnockbackInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM KnockbackInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, KnockbackInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &KnockbackInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO KnockbackInfo (
    Uid,
    KnockbackDir,
    KnockbackValue,
    KnockbackSpeed,
    IsGuard
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
    .bind(&data.knockback_dir_index)
    .bind(&data.knockback_value)
    .bind(&data.knockback_speed)
    .bind(&data.is_guard)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
