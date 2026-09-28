use crate::models::game::mini::mini_game_bingo_info::MiniGameBingoInfo;
use sqlx::SqlitePool;

pub async fn add_mini_game_bingo_info(pool: &SqlitePool, data: &MiniGameBingoInfo) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO MiniGameBingoInfo (Uid, EventScheduleId, ClearCount, BingoBoard, OpenBingoBoardIndex) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(data.uid)
    .bind(data.event_schedule_id)
    .bind(data.clear_count)
    .bind(&data.bingo_board)
    .bind(&data.open_bingo_board_index)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_mini_game_bingo_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MiniGameBingoInfo>> {
    sqlx::query_as::<_, MiniGameBingoInfo>("SELECT * FROM MiniGameBingoInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_schedule(pool: &SqlitePool, uid: i64, event_schedule_id: i32) -> sqlx::Result<Option<MiniGameBingoInfo>> {
    sqlx::query_as::<_, MiniGameBingoInfo>("SELECT * FROM MiniGameBingoInfo WHERE Uid = ? AND EventScheduleId = ?")
        .bind(uid)
        .bind(event_schedule_id)
        .fetch_optional(pool)
        .await
}

pub async fn update_progress(pool: &SqlitePool, index: i64, open_bingo_board_index: &str, clear_count: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE MiniGameBingoInfo SET OpenBingoBoardIndex = ?, ClearCount = ? WHERE \"Index\" = ?")
        .bind(open_bingo_board_index)
        .bind(clear_count)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all MiniGameBingoInfo rows for a UID.
pub async fn delete_mini_game_bingo_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameBingoInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
