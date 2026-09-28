use crate::models::game::notice::notice_contents_info::NoticeContentsInfo;
use sqlx::SqlitePool;

/// Add a single NoticeContentsInfo record from a Rust struct.
pub async fn add_notice_contents_info(
    pool: &SqlitePool,
    data: &NoticeContentsInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO NoticeContentsInfo (
    Uid,
    Type,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_notice_contents_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<NoticeContentsInfo>> {
    sqlx::query_as::<_, NoticeContentsInfo>("SELECT * FROM NoticeContentsInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all NoticeContentsInfo rows for a UID.
pub async fn delete_notice_contents_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM NoticeContentsInfo WHERE Uid = ?")
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
) -> sqlx::Result<NoticeContentsInfo> {
    sqlx::query_as::<_, NoticeContentsInfo>(
        "SELECT * FROM NoticeContentsInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<NoticeContentsInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM NoticeContentsInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, NoticeContentsInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &NoticeContentsInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO NoticeContentsInfo (
    Uid,
    Type,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.r#type)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
