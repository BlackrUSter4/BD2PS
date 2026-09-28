use crate::models::game::costume::costume_base_info::CostumeBaseInfo;
use sqlx::SqlitePool;

/// Add a single CostumeBaseInfo record from a Rust struct.
pub async fn add_costume_base_info(pool: &SqlitePool, data: &CostumeBaseInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CostumeBaseInfo (
    Uid,
    Id,
    Level,
    DesignId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.level)
    .bind(&data.design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_costume_base_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CostumeBaseInfo>> {
    sqlx::query_as::<_, CostumeBaseInfo>("SELECT * FROM CostumeBaseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CostumeBaseInfo rows for a UID.
pub async fn delete_costume_base_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CostumeBaseInfo WHERE Uid = ?")
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
) -> sqlx::Result<CostumeBaseInfo> {
    sqlx::query_as::<_, CostumeBaseInfo>(
        "SELECT * FROM CostumeBaseInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CostumeBaseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CostumeBaseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CostumeBaseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CostumeBaseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CostumeBaseInfo (
    Uid,
    Id,
    Level,
    DesignId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.level)
    .bind(&data.design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
