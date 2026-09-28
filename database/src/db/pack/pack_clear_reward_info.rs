use crate::models::game::pack::pack_clear_reward_info::PackClearRewardInfo;
use sqlx::SqlitePool;

/// Add a single PackClearRewardInfo record from a Rust struct.
pub async fn add_pack_clear_reward_info(
    pool: &SqlitePool,
    data: &PackClearRewardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackClearRewardInfo (
    Uid,
    GroupId,
    TicketId,
    PackId,
    PackLevel
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
    .bind(&data.pack_id)
    .bind(&data.pack_level)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_clear_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackClearRewardInfo>> {
    sqlx::query_as::<_, PackClearRewardInfo>("SELECT * FROM PackClearRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PackClearRewardInfo rows for a UID.
pub async fn delete_pack_clear_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackClearRewardInfo WHERE Uid = ?")
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
) -> sqlx::Result<PackClearRewardInfo> {
    sqlx::query_as::<_, PackClearRewardInfo>(
        "SELECT * FROM PackClearRewardInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PackClearRewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackClearRewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackClearRewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackClearRewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackClearRewardInfo (
    Uid,
    GroupId,
    TicketId,
    PackId,
    PackLevel
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
    .bind(&data.pack_id)
    .bind(&data.pack_level)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
