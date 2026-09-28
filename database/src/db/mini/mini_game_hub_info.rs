use crate::models::game::mini::mini_game_hub_info::MiniGameHubInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameHubInfo record from a Rust struct.
pub async fn add_mini_game_hub_info(pool: &SqlitePool, data: &MiniGameHubInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameHubInfo (
    Uid,
    Slot,
    EventUid,
    ProgressType
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.slot)
    .bind(&data.event_uid)
    .bind(&data.progress_type)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_hub_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameHubInfo>> {
    sqlx::query_as::<_, MiniGameHubInfo>("SELECT * FROM MiniGameHubInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MiniGameHubInfo rows for a UID.
pub async fn delete_mini_game_hub_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameHubInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameHubInfo> {
    sqlx::query_as::<_, MiniGameHubInfo>(
        "SELECT * FROM MiniGameHubInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameHubInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameHubInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameHubInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameHubInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameHubInfo (
    Uid,
    Slot,
    EventUid,
    ProgressType
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.slot)
    .bind(&data.event_uid)
    .bind(&data.progress_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
