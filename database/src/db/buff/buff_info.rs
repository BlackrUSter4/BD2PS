use crate::models::game::buff::buff_info::BuffInfo;
use sqlx::SqlitePool;

/// Add a single BuffInfo record from a Rust struct.
pub async fn add_buff_info(pool: &SqlitePool, data: &BuffInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BuffInfo (
    Uid,
    BuffId,
    CasterCharId,
    StartTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.buff_id)
    .bind(&data.caster_char_id)
    .bind(&data.start_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_buff_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<BuffInfo>> {
    sqlx::query_as::<_, BuffInfo>("SELECT * FROM BuffInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all BuffInfo rows for a UID.
pub async fn delete_buff_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BuffInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<BuffInfo> {
    sqlx::query_as::<_, BuffInfo>("SELECT * FROM BuffInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<BuffInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM BuffInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, BuffInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &BuffInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO BuffInfo (
    Uid,
    BuffId,
    CasterCharId,
    StartTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.buff_id)
    .bind(&data.caster_char_id)
    .bind(&data.start_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
