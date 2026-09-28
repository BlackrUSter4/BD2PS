use crate::models::game::popular::popular_costume_count_info::PopularCostumeCountInfo;
use sqlx::SqlitePool;

/// Add a single PopularCostumeCountInfo record from a Rust struct.
pub async fn add_popular_costume_count_info(
    pool: &SqlitePool,
    data: &PopularCostumeCountInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PopularCostumeCountInfo (
    Uid,
    Id,
    CountType0,
    CountType1,
    CountType2
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
    .bind(&data.id)
    .bind(&data.count_type_0)
    .bind(&data.count_type_1)
    .bind(&data.count_type_2)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_popular_costume_count_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PopularCostumeCountInfo>> {
    sqlx::query_as::<_, PopularCostumeCountInfo>(
        "SELECT * FROM PopularCostumeCountInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all PopularCostumeCountInfo rows for a UID.
pub async fn delete_popular_costume_count_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PopularCostumeCountInfo WHERE Uid = ?")
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
) -> sqlx::Result<PopularCostumeCountInfo> {
    sqlx::query_as::<_, PopularCostumeCountInfo>(
        "SELECT * FROM PopularCostumeCountInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PopularCostumeCountInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PopularCostumeCountInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PopularCostumeCountInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PopularCostumeCountInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PopularCostumeCountInfo (
    Uid,
    Id,
    CountType0,
    CountType1,
    CountType2
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
    .bind(&data.id)
    .bind(&data.count_type_0)
    .bind(&data.count_type_1)
    .bind(&data.count_type_2)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
