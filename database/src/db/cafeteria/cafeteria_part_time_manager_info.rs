use crate::models::game::cafeteria::cafeteria_part_time_manager_info::CafeteriaPartTimeManagerInfo;
use sqlx::SqlitePool;

/// Add a single CafeteriaPartTimeManagerInfo record from a Rust struct.
pub async fn add_cafeteria_part_time_manager_info(
    pool: &SqlitePool,
    data: &CafeteriaPartTimeManagerInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CafeteriaPartTimeManagerInfo (
    Uid,
    PartTimeManagerId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.part_time_manager_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cafeteria_part_time_manager_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CafeteriaPartTimeManagerInfo>> {
    sqlx::query_as::<_, CafeteriaPartTimeManagerInfo>(
        "SELECT * FROM CafeteriaPartTimeManagerInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all CafeteriaPartTimeManagerInfo rows for a UID.
pub async fn delete_cafeteria_part_time_manager_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CafeteriaPartTimeManagerInfo WHERE Uid = ?")
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
) -> sqlx::Result<CafeteriaPartTimeManagerInfo> {
    sqlx::query_as::<_, CafeteriaPartTimeManagerInfo>(
        "SELECT * FROM CafeteriaPartTimeManagerInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CafeteriaPartTimeManagerInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CafeteriaPartTimeManagerInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CafeteriaPartTimeManagerInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CafeteriaPartTimeManagerInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CafeteriaPartTimeManagerInfo (
    Uid,
    PartTimeManagerId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.part_time_manager_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
