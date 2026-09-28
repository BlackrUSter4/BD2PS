use crate::models::game::pack::pack_event_battle_info::PackEventBattleInfo;
use sqlx::SqlitePool;

/// Add a single PackEventBattleInfo record from a Rust struct.
pub async fn add_pack_event_battle_info(
    pool: &SqlitePool,
    data: &PackEventBattleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackEventBattleInfo (
    Uid,
    EventUid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_event_battle_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackEventBattleInfo>> {
    sqlx::query_as::<_, PackEventBattleInfo>("SELECT * FROM PackEventBattleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PackEventBattleInfo rows for a UID.
pub async fn delete_pack_event_battle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackEventBattleInfo WHERE Uid = ?")
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
) -> sqlx::Result<PackEventBattleInfo> {
    sqlx::query_as::<_, PackEventBattleInfo>(
        "SELECT * FROM PackEventBattleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PackEventBattleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackEventBattleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackEventBattleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackEventBattleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackEventBattleInfo (
    Uid,
    EventUid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
