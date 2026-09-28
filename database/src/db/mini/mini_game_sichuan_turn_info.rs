use crate::models::game::mini::mini_game_sichuan_turn_info::MiniGameSichuanTurnInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSichuanTurnInfo record from a Rust struct.
pub async fn add_mini_game_sichuan_turn_info(
    pool: &SqlitePool,
    data: &MiniGameSichuanTurnInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSichuanTurnInfo (
    Uid,
    Turn
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.turn)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_sichuan_turn_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSichuanTurnInfo>> {
    sqlx::query_as::<_, MiniGameSichuanTurnInfo>(
        "SELECT * FROM MiniGameSichuanTurnInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameSichuanTurnInfo rows for a UID.
pub async fn delete_mini_game_sichuan_turn_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSichuanTurnInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSichuanTurnInfo> {
    sqlx::query_as::<_, MiniGameSichuanTurnInfo>(
        "SELECT * FROM MiniGameSichuanTurnInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSichuanTurnInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSichuanTurnInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSichuanTurnInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSichuanTurnInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSichuanTurnInfo (
    Uid,
    Turn
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.turn)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
