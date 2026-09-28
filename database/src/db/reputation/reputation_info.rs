use crate::models::game::reputation::reputation_info::ReputationInfo;
use sqlx::SqlitePool;

/// Add a single ReputationInfo record from a Rust struct.
pub async fn add_reputation_info(pool: &SqlitePool, data: &ReputationInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ReputationInfo (
    Uid,
    GroupId,
    State,
    ElapsedSeconds
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.state)
    .bind(&data.elapsed_seconds)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_reputation_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ReputationInfo>> {
    sqlx::query_as::<_, ReputationInfo>("SELECT * FROM ReputationInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ReputationInfo rows for a UID.
pub async fn delete_reputation_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ReputationInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ReputationInfo> {
    sqlx::query_as::<_, ReputationInfo>("SELECT * FROM ReputationInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ReputationInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ReputationInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ReputationInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ReputationInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ReputationInfo (
    Uid,
    GroupId,
    State,
    ElapsedSeconds
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.state)
    .bind(&data.elapsed_seconds)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
