use crate::models::game::reward::reward_info::RewardInfo;
use sqlx::SqlitePool;

/// Add a single RewardInfo record from a Rust struct.
pub async fn add_reward_info(pool: &SqlitePool, data: &RewardInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RewardInfo (
    Uid,
    ItemId,
    ItemType,
    ItemCount
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.item_id)
    .bind(&data.item_type)
    .bind(&data.item_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<RewardInfo>> {
    sqlx::query_as::<_, RewardInfo>("SELECT * FROM RewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RewardInfo rows for a UID.
pub async fn delete_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RewardInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<RewardInfo> {
    sqlx::query_as::<_, RewardInfo>("SELECT * FROM RewardInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<RewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RewardInfo (
    Uid,
    ItemId,
    ItemType,
    ItemCount
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.item_id)
    .bind(&data.item_type)
    .bind(&data.item_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
