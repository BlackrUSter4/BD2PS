use crate::models::game::content::content_rank_statue_info::ContentRankStatueInfo;
use sqlx::SqlitePool;

/// Add a single ContentRankStatueInfo record from a Rust struct.
pub async fn add_content_rank_statue_info(
    pool: &SqlitePool,
    data: &ContentRankStatueInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ContentRankStatueInfo (
    Uid,
    Id,
    Season,
    ErrorFlag,
    StatueGroupInfoIndex
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
    .bind(&data.season)
    .bind(&data.error_flag)
    .bind(&data.statue_group_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_content_rank_statue_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ContentRankStatueInfo>> {
    sqlx::query_as::<_, ContentRankStatueInfo>("SELECT * FROM ContentRankStatueInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ContentRankStatueInfo rows for a UID.
pub async fn delete_content_rank_statue_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ContentRankStatueInfo WHERE Uid = ?")
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
) -> sqlx::Result<ContentRankStatueInfo> {
    sqlx::query_as::<_, ContentRankStatueInfo>(
        "SELECT * FROM ContentRankStatueInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ContentRankStatueInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ContentRankStatueInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ContentRankStatueInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ContentRankStatueInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ContentRankStatueInfo (
    Uid,
    Id,
    Season,
    ErrorFlag,
    StatueGroupInfoIndex
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
    .bind(&data.season)
    .bind(&data.error_flag)
    .bind(&data.statue_group_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
