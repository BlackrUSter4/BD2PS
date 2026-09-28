use crate::models::game::personal::personal_info::PersonalInfo;
use sqlx::SqlitePool;

/// Add a single PersonalInfo record from a Rust struct.
pub async fn add_personal_info(pool: &SqlitePool, data: &PersonalInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PersonalInfo (
    Uid,
    Id,
    Title,
    Url,
    StartDate,
    EndDate
) VALUES (
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
    .bind(&data.title)
    .bind(&data.url)
    .bind(&data.start_date)
    .bind(&data.end_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_personal_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PersonalInfo>> {
    sqlx::query_as::<_, PersonalInfo>("SELECT * FROM PersonalInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PersonalInfo rows for a UID.
pub async fn delete_personal_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PersonalInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<PersonalInfo> {
    sqlx::query_as::<_, PersonalInfo>("SELECT * FROM PersonalInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<PersonalInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PersonalInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PersonalInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PersonalInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PersonalInfo (
    Uid,
    Id,
    Title,
    Url,
    StartDate,
    EndDate
) VALUES (
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
    .bind(&data.title)
    .bind(&data.url)
    .bind(&data.start_date)
    .bind(&data.end_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
