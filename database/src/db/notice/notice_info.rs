use crate::models::game::notice::notice_info::NoticeInfo;
use sqlx::SqlitePool;

/// Add a single NoticeInfo record from a Rust struct.
pub async fn add_notice_info(pool: &SqlitePool, data: &NoticeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO NoticeInfo (
    Uid,
    Id,
    NoticeType,
    Title,
    Thumbnail,
    StartTime,
    EndTime,
    WebUrl,
    PromotionBannerId,
    IsPin,
    Sort,
    SubType,
    IsInvert
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.notice_type)
    .bind(&data.title)
    .bind(&data.thumbnail)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.web_url)
    .bind(&data.promotion_banner_id)
    .bind(&data.is_pin)
    .bind(&data.sort)
    .bind(&data.sub_type)
    .bind(&data.is_invert)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_notice_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<NoticeInfo>> {
    sqlx::query_as::<_, NoticeInfo>("SELECT * FROM NoticeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all NoticeInfo rows for a UID.
pub async fn delete_notice_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM NoticeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<NoticeInfo> {
    sqlx::query_as::<_, NoticeInfo>("SELECT * FROM NoticeInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<NoticeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM NoticeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, NoticeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &NoticeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO NoticeInfo (
    Uid,
    Id,
    NoticeType,
    Title,
    Thumbnail,
    StartTime,
    EndTime,
    WebUrl,
    PromotionBannerId,
    IsPin,
    Sort,
    SubType,
    IsInvert
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.notice_type)
    .bind(&data.title)
    .bind(&data.thumbnail)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.web_url)
    .bind(&data.promotion_banner_id)
    .bind(&data.is_pin)
    .bind(&data.sort)
    .bind(&data.sub_type)
    .bind(&data.is_invert)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
