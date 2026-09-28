use crate::models::game::mini::mini_game_bingo_line_info::MiniGameBingoLineInfo;
use sqlx::SqlitePool;

pub async fn add_mini_game_bingo_line_info(pool: &SqlitePool, data: &MiniGameBingoLineInfo) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO MiniGameBingoLineInfo (Uid, EventScheduleId, LineType, LineIndex) VALUES (?, ?, ?, ?)")
        .bind(data.uid)
        .bind(data.event_schedule_id)
        .bind(data.line_type)
        .bind(data.line_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_mini_game_bingo_line_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameBingoLineInfo>> {
    sqlx::query_as::<_, MiniGameBingoLineInfo>("SELECT * FROM MiniGameBingoLineInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_schedule(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<Vec<MiniGameBingoLineInfo>> {
    sqlx::query_as::<_, MiniGameBingoLineInfo>("SELECT * FROM MiniGameBingoLineInfo WHERE Uid = ? AND EventScheduleId = ?")
        .bind(uid)
        .bind(event_schedule_id)
        .fetch_all(pool)
        .await
}

pub async fn is_line_open(pool: &SqlitePool, uid: i64, event_schedule_id: i32, line_type: i32, line_index: i32) -> sqlx::Result<bool> {
    let row = sqlx::query_as::<_, MiniGameBingoLineInfo>(
        "SELECT * FROM MiniGameBingoLineInfo WHERE Uid = ? AND EventScheduleId = ? AND LineType = ? AND LineIndex = ?",
    )
    .bind(uid)
    .bind(event_schedule_id)
    .bind(line_type)
    .bind(line_index)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Delete all MiniGameBingoLineInfo rows for a UID.
pub async fn delete_mini_game_bingo_line_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameBingoLineInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
