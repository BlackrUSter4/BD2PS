use crate::models::game::mini::mini_game_board_info::MiniGameBoardInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameBoardInfo record from a Rust struct.
pub async fn add_mini_game_board_info(
    pool: &SqlitePool,
    data: &MiniGameBoardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameBoardInfo (
    Uid,
    EventScheduleId,
    ScaffoldGroupId,
    ScaffoldId,
    CompleteCount
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
    .bind(&data.event_schedule_id)
    .bind(&data.scaffold_group_id)
    .bind(&data.scaffold_id)
    .bind(&data.complete_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_board_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameBoardInfo>> {
    sqlx::query_as::<_, MiniGameBoardInfo>("SELECT * FROM MiniGameBoardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_schedule(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<Option<MiniGameBoardInfo>> {
    sqlx::query_as::<_, MiniGameBoardInfo>("SELECT * FROM MiniGameBoardInfo WHERE Uid = ? AND EventScheduleId = ?")
        .bind(uid)
        .bind(event_schedule_id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_progress(pool: &SqlitePool, uid: i64, event_schedule_id: i32, scaffold_group_id: i32, scaffold_id: i32, complete_count: i32) -> sqlx::Result<()> {
    if let Some(row) = get_by_schedule(pool, uid, event_schedule_id).await? {
        sqlx::query("UPDATE MiniGameBoardInfo SET ScaffoldId = ?, CompleteCount = ? WHERE \"Index\" = ?")
            .bind(scaffold_id)
            .bind(complete_count)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_board_info(pool, &MiniGameBoardInfo { index: 0, uid, event_schedule_id: Some(event_schedule_id), scaffold_group_id: Some(scaffold_group_id), scaffold_id: Some(scaffold_id), complete_count: Some(complete_count) }).await?;
    }
    Ok(())
}

/// Delete all MiniGameBoardInfo rows for a UID.
pub async fn delete_mini_game_board_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameBoardInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameBoardInfo> {
    sqlx::query_as::<_, MiniGameBoardInfo>(
        "SELECT * FROM MiniGameBoardInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameBoardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameBoardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameBoardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameBoardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameBoardInfo (
    Uid,
    EventScheduleId,
    ScaffoldGroupId,
    ScaffoldId,
    CompleteCount
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
    .bind(&data.event_schedule_id)
    .bind(&data.scaffold_group_id)
    .bind(&data.scaffold_id)
    .bind(&data.complete_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
