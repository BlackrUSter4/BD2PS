use crate::models::game::ib::ib_cleared_dungeon::IbClearedDungeon;
use sqlx::SqlitePool;

pub async fn list_for_season(pool: &SqlitePool, uid: i64, season: i32) -> sqlx::Result<Vec<i32>> {
    let rows: Vec<(i32,)> = sqlx::query_as(
        "SELECT DungeonId FROM IbClearedDungeon WHERE Uid = ? AND Season = ?",
    )
    .bind(uid)
    .bind(season)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

/// Returns true if this is the first time this dungeon was cleared this season.
pub async fn mark_cleared(pool: &SqlitePool, uid: i64, season: i32, dungeon_id: i32) -> sqlx::Result<bool> {
    let existing: Option<(i32,)> = sqlx::query_as(
        "SELECT DungeonId FROM IbClearedDungeon WHERE Uid = ? AND Season = ? AND DungeonId = ?",
    )
    .bind(uid)
    .bind(season)
    .bind(dungeon_id)
    .fetch_optional(pool)
    .await?;
    if existing.is_some() {
        return Ok(false);
    }
    sqlx::query("INSERT INTO IbClearedDungeon (Uid, Season, DungeonId) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(season)
        .bind(dungeon_id)
        .execute(pool)
        .await?;
    Ok(true)
}

#[allow(dead_code)]
pub async fn clear_all_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IbClearedDungeon WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn all(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<IbClearedDungeon>> {
    sqlx::query_as::<_, IbClearedDungeon>("SELECT * FROM IbClearedDungeon WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}
