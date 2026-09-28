use crate::models::game::mini::mini_puzzle_info::MiniPuzzleInfo;
use sqlx::SqlitePool;

/// Add a single MiniPuzzleInfo record from a Rust struct.
pub async fn add_mini_puzzle_info(pool: &SqlitePool, data: &MiniPuzzleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniPuzzleInfo (
    Uid,
    EventScheduleId,
    ClearCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_schedule_id)
    .bind(&data.clear_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_puzzle_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniPuzzleInfo>> {
    sqlx::query_as::<_, MiniPuzzleInfo>("SELECT * FROM MiniPuzzleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_schedule(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<Option<MiniPuzzleInfo>> {
    sqlx::query_as::<_, MiniPuzzleInfo>("SELECT * FROM MiniPuzzleInfo WHERE Uid = ? AND EventScheduleId = ?")
        .bind(uid)
        .bind(event_schedule_id)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<MiniPuzzleInfo> {
    if let Some(row) = get_by_schedule(pool, uid, event_schedule_id).await? {
        return Ok(row);
    }
    add_mini_puzzle_info(pool, &MiniPuzzleInfo { index: 0, uid, event_schedule_id: Some(event_schedule_id), clear_count: Some(0), puzzle: 0, puzzle_open: 0 }).await?;
    Ok(get_by_schedule(pool, uid, event_schedule_id).await?.expect("row was just inserted"))
}

pub async fn update_open(pool: &SqlitePool, index: i64, puzzle_open: i32, clear_count: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE MiniPuzzleInfo SET PuzzleOpen = ?, ClearCount = ? WHERE \"Index\" = ?")
        .bind(puzzle_open)
        .bind(clear_count)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all MiniPuzzleInfo rows for a UID.
pub async fn delete_mini_puzzle_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniPuzzleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MiniPuzzleInfo> {
    sqlx::query_as::<_, MiniPuzzleInfo>("SELECT * FROM MiniPuzzleInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MiniPuzzleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniPuzzleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniPuzzleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniPuzzleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniPuzzleInfo (
    Uid,
    EventScheduleId,
    ClearCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.event_schedule_id)
    .bind(&data.clear_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
