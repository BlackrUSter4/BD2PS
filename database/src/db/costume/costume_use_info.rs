use crate::models::game::costume::costume_use_info::CostumeUseInfo;
use sqlx::SqlitePool;

/// Add a single CostumeUseInfo record from a Rust struct.
pub async fn add_costume_use_info(pool: &SqlitePool, data: &CostumeUseInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CostumeUseInfo (
    Uid,
    CostumeIndex,
    CharIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_index)
    .bind(&data.char_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_costume_use_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CostumeUseInfo>> {
    sqlx::query_as::<_, CostumeUseInfo>("SELECT * FROM CostumeUseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CostumeUseInfo rows for a UID.
pub async fn delete_costume_use_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CostumeUseInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CostumeUseInfo> {
    sqlx::query_as::<_, CostumeUseInfo>("SELECT * FROM CostumeUseInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<CostumeUseInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CostumeUseInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CostumeUseInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CostumeUseInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CostumeUseInfo (
    Uid,
    CostumeIndex,
    CharIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_index)
    .bind(&data.char_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
