use crate::models::game::pictorial::pictorial_book_info::PictorialBookInfo;
use sqlx::SqlitePool;

/// Add a single PictorialBookInfo record from a Rust struct.
pub async fn add_pictorial_book_info(
    pool: &SqlitePool,
    data: &PictorialBookInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PictorialBookInfo (
    Uid,
    Id,
    GroupId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.group_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pictorial_book_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PictorialBookInfo>> {
    sqlx::query_as::<_, PictorialBookInfo>("SELECT * FROM PictorialBookInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PictorialBookInfo rows for a UID.
pub async fn delete_pictorial_book_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PictorialBookInfo WHERE Uid = ?")
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
) -> sqlx::Result<PictorialBookInfo> {
    sqlx::query_as::<_, PictorialBookInfo>(
        "SELECT * FROM PictorialBookInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PictorialBookInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PictorialBookInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PictorialBookInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PictorialBookInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PictorialBookInfo (
    Uid,
    Id,
    GroupId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.group_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
