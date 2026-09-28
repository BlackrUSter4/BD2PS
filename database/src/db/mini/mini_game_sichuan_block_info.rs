use crate::models::game::mini::mini_game_sichuan_block_info::MiniGameSichuanBlockInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSichuanBlockInfo record from a Rust struct.
pub async fn add_mini_game_sichuan_block_info(
    pool: &SqlitePool,
    data: &MiniGameSichuanBlockInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSichuanBlockInfo (
    Uid,
    Id,
    Count
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_sichuan_block_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSichuanBlockInfo>> {
    sqlx::query_as::<_, MiniGameSichuanBlockInfo>(
        "SELECT * FROM MiniGameSichuanBlockInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameSichuanBlockInfo rows for a UID.
pub async fn delete_mini_game_sichuan_block_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSichuanBlockInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSichuanBlockInfo> {
    sqlx::query_as::<_, MiniGameSichuanBlockInfo>(
        "SELECT * FROM MiniGameSichuanBlockInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSichuanBlockInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSichuanBlockInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSichuanBlockInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSichuanBlockInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSichuanBlockInfo (
    Uid,
    Id,
    Count
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
