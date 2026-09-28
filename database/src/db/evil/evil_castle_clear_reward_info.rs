use crate::models::game::evil::evil_castle_clear_reward_info::EvilCastleClearRewardInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleClearRewardInfo record from a Rust struct.
pub async fn add_evil_castle_clear_reward_info(
    pool: &SqlitePool,
    data: &EvilCastleClearRewardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleClearRewardInfo (
    Uid,
    GroupId,
    TicketId,
    TowerType,
    TowerMagicId
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
    .bind(&data.ticket_id)
    .bind(&data.tower_type)
    .bind(&data.tower_magic_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_clear_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleClearRewardInfo>> {
    sqlx::query_as::<_, EvilCastleClearRewardInfo>(
        "SELECT * FROM EvilCastleClearRewardInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all EvilCastleClearRewardInfo rows for a UID.
pub async fn delete_evil_castle_clear_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleClearRewardInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleClearRewardInfo> {
    sqlx::query_as::<_, EvilCastleClearRewardInfo>(
        "SELECT * FROM EvilCastleClearRewardInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleClearRewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleClearRewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleClearRewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleClearRewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleClearRewardInfo (
    Uid,
    GroupId,
    TicketId,
    TowerType,
    TowerMagicId
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
    .bind(&data.ticket_id)
    .bind(&data.tower_type)
    .bind(&data.tower_magic_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
