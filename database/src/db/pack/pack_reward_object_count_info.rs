use crate::models::game::pack::pack_reward_object_count_info::PackRewardObjectCountInfo;
use sqlx::SqlitePool;

/// Add a single PackRewardObjectCountInfo record from a Rust struct.
pub async fn add_pack_reward_object_count_info(
    pool: &SqlitePool,
    data: &PackRewardObjectCountInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackRewardObjectCountInfo (
    Uid,
    Type,
    PackId,
    Count,
    MaxCount
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
    .bind(&data.r#type)
    .bind(&data.pack_id)
    .bind(&data.count)
    .bind(&data.max_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_reward_object_count_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackRewardObjectCountInfo>> {
    sqlx::query_as::<_, PackRewardObjectCountInfo>(
        "SELECT * FROM PackRewardObjectCountInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Get the row for one (pack_id, type) pair, if saved.
pub async fn get_by_uid_pack_type(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
    r#type: i32,
) -> sqlx::Result<Option<PackRewardObjectCountInfo>> {
    sqlx::query_as::<_, PackRewardObjectCountInfo>(
        "SELECT * FROM PackRewardObjectCountInfo WHERE Uid = ? AND PackId = ? AND Type = ?",
    )
    .bind(uid)
    .bind(pack_id)
    .bind(r#type)
    .fetch_optional(pool)
    .await
}

/// Get an existing (pack_id, type) count row, or create one at count 0 (max_count from
/// FieldRewardObjectGroupTable-style caps isn't modeled here — real max comes from
/// whatever the caller passes in).
pub async fn get_or_create(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
    r#type: i32,
    max_count: i32,
) -> sqlx::Result<PackRewardObjectCountInfo> {
    if let Some(row) = get_by_uid_pack_type(pool, uid, pack_id, r#type).await? {
        return Ok(row);
    }
    sqlx::query(
        "INSERT INTO PackRewardObjectCountInfo (Uid, Type, PackId, Count, MaxCount) VALUES (?, ?, ?, 0, ?)",
    )
    .bind(uid)
    .bind(r#type)
    .bind(pack_id)
    .bind(max_count)
    .execute(pool)
    .await?;
    Ok(get_by_uid_pack_type(pool, uid, pack_id, r#type)
        .await?
        .expect("row was just inserted"))
}

/// Delete all PackRewardObjectCountInfo rows for a UID.
pub async fn delete_pack_reward_object_count_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackRewardObjectCountInfo WHERE Uid = ?")
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
) -> sqlx::Result<PackRewardObjectCountInfo> {
    sqlx::query_as::<_, PackRewardObjectCountInfo>(
        "SELECT * FROM PackRewardObjectCountInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PackRewardObjectCountInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackRewardObjectCountInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackRewardObjectCountInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackRewardObjectCountInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackRewardObjectCountInfo (
    Uid,
    Type,
    PackId,
    Count,
    MaxCount
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
    .bind(&data.r#type)
    .bind(&data.pack_id)
    .bind(&data.count)
    .bind(&data.max_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
