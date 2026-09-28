use crate::models::game::ib::ib_play_state::IbPlayState;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<IbPlayState>> {
    sqlx::query_as::<_, IbPlayState>("SELECT * FROM IbPlayState WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<IbPlayState> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = IbPlayState { uid, ..Default::default() };
    insert(pool, &data).await?;
    Ok(data)
}

pub async fn insert(pool: &SqlitePool, data: &IbPlayState) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO IbPlayState (
    Uid, DungeonId, StageId, Life, Coin, Season, ShopReloadCount,
    IsStageEnter, CurrentBattleMode, CurrentRandomSeed
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.dungeon_id)
    .bind(data.stage_id)
    .bind(data.life)
    .bind(data.coin)
    .bind(data.season)
    .bind(data.shop_reload_count)
    .bind(data.is_stage_enter)
    .bind(data.current_battle_mode)
    .bind(data.current_random_seed)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn enter_dungeon(
    pool: &SqlitePool,
    uid: i64,
    dungeon_id: i32,
    starting_life: i32,
) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query(
        "UPDATE IbPlayState SET DungeonId = ?, StageId = 1, Life = ?, IsStageEnter = 0, CurrentBattleMode = NULL, CurrentRandomSeed = NULL WHERE Uid = ?",
    )
    .bind(dungeon_id)
    .bind(starting_life)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn give_up(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query(
        "UPDATE IbPlayState SET DungeonId = 0, StageId = 0, IsStageEnter = 0, CurrentBattleMode = NULL, CurrentRandomSeed = NULL WHERE Uid = ?",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn start_stage(pool: &SqlitePool, uid: i64, battle_mode: i32, random_seed: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query(
        "UPDATE IbPlayState SET IsStageEnter = 1, CurrentBattleMode = ?, CurrentRandomSeed = ? WHERE Uid = ?",
    )
    .bind(battle_mode)
    .bind(random_seed)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Applies a stage result: win advances (dungeon considered cleared — no per-dungeon stage
/// count data is captured, see gameserver/src/logic/game/ib/mod.rs), loss costs one life.
/// Returns (new_life, new_coin, next_stage_id).
pub async fn end_stage(
    pool: &SqlitePool,
    uid: i64,
    won: bool,
    gain_coin: i32,
) -> sqlx::Result<(i32, i32, i32)> {
    let state = get_or_create(pool, uid).await?;
    let new_life = if won { state.life } else { (state.life - 1).max(0) };
    let new_coin = (state.coin + gain_coin).max(0);
    let next_stage_id = if won { 0 } else { state.stage_id };

    sqlx::query(
        "UPDATE IbPlayState SET Life = ?, Coin = ?, StageId = ?, IsStageEnter = 0, CurrentBattleMode = NULL, CurrentRandomSeed = NULL WHERE Uid = ?",
    )
    .bind(new_life)
    .bind(new_coin)
    .bind(next_stage_id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok((new_life, new_coin, next_stage_id))
}

pub async fn add_coin(pool: &SqlitePool, uid: i64, delta: i32) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE IbPlayState SET Coin = MAX(0, Coin + ?) WHERE Uid = ?")
        .bind(delta)
        .bind(uid)
        .execute(pool)
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT Coin FROM IbPlayState WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn incr_shop_reload(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE IbPlayState SET ShopReloadCount = ShopReloadCount + 1 WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT ShopReloadCount FROM IbPlayState WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Resets dungeon progress for a new season (coin/inventory/deck are kept — no captured data
/// confirms whether these should wipe on season change, so the less-destructive choice was
/// made; revisit if real `IBSeasonTable` data is ever captured).
pub async fn reset_for_new_season(pool: &SqlitePool, uid: i64, new_season: i32, starting_life: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query(
        "UPDATE IbPlayState SET Season = ?, DungeonId = 0, StageId = 0, Life = ?, IsStageEnter = 0, ShopReloadCount = 0, CurrentBattleMode = NULL, CurrentRandomSeed = NULL WHERE Uid = ?",
    )
    .bind(new_season)
    .bind(starting_life)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
