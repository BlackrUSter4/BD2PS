use crate::models::game::achievement::achievement_clear_info::AchievementClearInfo;
use sqlx::SqlitePool;

/// Add a single AchievementClearInfo record from a Rust struct.
pub async fn add_achievement_clear_info(
    pool: &SqlitePool,
    data: &AchievementClearInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AchievementClearInfo (
    Uid,
    GroupId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_achievement_clear_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<AchievementClearInfo>> {
    sqlx::query_as::<_, AchievementClearInfo>("SELECT * FROM AchievementClearInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all AchievementClearInfo rows for a UID.
pub async fn delete_achievement_clear_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM AchievementClearInfo WHERE Uid = ?")
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
) -> sqlx::Result<AchievementClearInfo> {
    sqlx::query_as::<_, AchievementClearInfo>(
        "SELECT * FROM AchievementClearInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<AchievementClearInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM AchievementClearInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, AchievementClearInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &AchievementClearInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO AchievementClearInfo (
    Uid,
    GroupId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
