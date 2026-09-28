use crate::models::game::achievement::achievement_info::AchievementInfo;
use sqlx::SqlitePool;

/// Add a single AchievementInfo record from a Rust struct.
pub async fn add_achievement_info(pool: &SqlitePool, data: &AchievementInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AchievementInfo (
    Uid,
    GroupId,
    Value,
    MaxClearId,
    ContentsGroup
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
    .bind(&data.group_id)
    .bind(&data.value)
    .bind(&data.max_clear_id)
    .bind(&data.contents_group)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_achievement_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<AchievementInfo>> {
    sqlx::query_as::<_, AchievementInfo>("SELECT * FROM AchievementInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_group(pool: &SqlitePool, uid: i64, group_id: i32) -> sqlx::Result<Option<AchievementInfo>> {
    sqlx::query_as::<_, AchievementInfo>("SELECT * FROM AchievementInfo WHERE Uid = ? AND GroupId = ?")
        .bind(uid)
        .bind(group_id)
        .fetch_optional(pool)
        .await
}

pub async fn add_value(pool: &SqlitePool, uid: i64, group_id: i32, contents_group: Option<i32>, delta: i64) -> sqlx::Result<()> {
    if get_by_group(pool, uid, group_id).await?.is_some() {
        sqlx::query("UPDATE AchievementInfo SET Value = COALESCE(Value, 0) + ? WHERE Uid = ? AND GroupId = ?")
            .bind(delta)
            .bind(uid)
            .bind(group_id)
            .execute(pool)
            .await?;
    } else {
        add_achievement_info(
            pool,
            &AchievementInfo { index: 0, uid, group_id: Some(group_id), value: Some(delta), max_clear_id: Some(0), contents_group },
        )
        .await?;
    }
    Ok(())
}

pub async fn set_max_clear_id(pool: &SqlitePool, uid: i64, group_id: i32, max_clear_id: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE AchievementInfo SET MaxClearId = ? WHERE Uid = ? AND GroupId = ?")
        .bind(max_clear_id)
        .bind(uid)
        .bind(group_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all AchievementInfo rows for a UID.
pub async fn delete_achievement_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM AchievementInfo WHERE Uid = ?")
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
) -> sqlx::Result<AchievementInfo> {
    sqlx::query_as::<_, AchievementInfo>(
        "SELECT * FROM AchievementInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<AchievementInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM AchievementInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, AchievementInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &AchievementInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO AchievementInfo (
    Uid,
    GroupId,
    Value,
    MaxClearId,
    ContentsGroup
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
    .bind(&data.group_id)
    .bind(&data.value)
    .bind(&data.max_clear_id)
    .bind(&data.contents_group)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
