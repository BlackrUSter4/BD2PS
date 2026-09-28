use crate::models::game::mini::mini_game_move_controller_info::MiniGameMoveControllerInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameMoveControllerInfo record from a Rust struct.
pub async fn add_mini_game_move_controller_info(
    pool: &SqlitePool,
    data: &MiniGameMoveControllerInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameMoveControllerInfo (
    Uid,
    Id,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_move_controller_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameMoveControllerInfo>> {
    sqlx::query_as::<_, MiniGameMoveControllerInfo>(
        "SELECT * FROM MiniGameMoveControllerInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameMoveControllerInfo rows for a UID.
pub async fn delete_mini_game_move_controller_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameMoveControllerInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameMoveControllerInfo> {
    sqlx::query_as::<_, MiniGameMoveControllerInfo>(
        "SELECT * FROM MiniGameMoveControllerInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameMoveControllerInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameMoveControllerInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameMoveControllerInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameMoveControllerInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameMoveControllerInfo (
    Uid,
    Id,
    Value
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.value)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
