use crate::models::game::colosseum::colosseum_user_info::ColosseumUserInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<ColosseumUserInfo>> {
    sqlx::query_as::<_, ColosseumUserInfo>("SELECT * FROM ColosseumUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<ColosseumUserInfo> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = ColosseumUserInfo { uid, ..Default::default() };
    insert(pool, &data).await?;
    Ok(data)
}

pub async fn insert(pool: &SqlitePool, data: &ColosseumUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ColosseumUserInfo (
    Uid, Vp, WinCount, LoseCount, Season, ApBuyCount, ApBuyResetTime,
    BattleCountResetTime, MatchRerollCount, CurrentBattleEnemyIndex
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.vp)
    .bind(data.win_count)
    .bind(data.lose_count)
    .bind(data.season)
    .bind(data.ap_buy_count)
    .bind(data.ap_buy_reset_time)
    .bind(data.battle_count_reset_time)
    .bind(data.match_reroll_count)
    .bind(data.current_battle_enemy_index)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_current_battle_enemy(
    pool: &SqlitePool,
    uid: i64,
    enemy_owner_index: Option<i64>,
) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE ColosseumUserInfo SET CurrentBattleEnemyIndex = ? WHERE Uid = ?")
        .bind(enemy_owner_index)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Applies a battle result: adjusts VP (clamped at 0) and increments win/lose count.
pub async fn apply_battle_result(
    pool: &SqlitePool,
    uid: i64,
    vp_delta: i32,
    won: bool,
) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    if won {
        sqlx::query(
            "UPDATE ColosseumUserInfo SET Vp = MAX(0, Vp + ?), WinCount = WinCount + 1 WHERE Uid = ?",
        )
    } else {
        sqlx::query(
            "UPDATE ColosseumUserInfo SET Vp = MAX(0, Vp + ?), LoseCount = LoseCount + 1 WHERE Uid = ?",
        )
    }
    .bind(vp_delta)
    .bind(uid)
    .execute(pool)
    .await?;
    let row: (i32,) = sqlx::query_as("SELECT Vp FROM ColosseumUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn set_ap_buy(pool: &SqlitePool, uid: i64, count: i32, reset_time: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE ColosseumUserInfo SET ApBuyCount = ?, ApBuyResetTime = ? WHERE Uid = ?")
        .bind(count)
        .bind(reset_time)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_battle_count_reset(pool: &SqlitePool, uid: i64, time: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE ColosseumUserInfo SET BattleCountResetTime = ? WHERE Uid = ?")
        .bind(time)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn incr_match_reroll(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE ColosseumUserInfo SET MatchRerollCount = MatchRerollCount + 1 WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT MatchRerollCount FROM ColosseumUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn advance_season(pool: &SqlitePool, uid: i64, new_season: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE ColosseumUserInfo SET Season = ? WHERE Uid = ?")
        .bind(new_season)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Opponents near the caller's VP, excluding the caller. Real accounts only —
/// bot synthesis (when too few/no other real accounts exist) happens in gameserver logic.
pub async fn find_nearby_opponents(
    pool: &SqlitePool,
    uid: i64,
    limit: i32,
) -> sqlx::Result<Vec<ColosseumUserInfo>> {
    sqlx::query_as::<_, ColosseumUserInfo>(
        r#"
SELECT * FROM ColosseumUserInfo
WHERE Uid != ?
ORDER BY ABS(Vp - (SELECT Vp FROM ColosseumUserInfo WHERE Uid = ?))
LIMIT ?
"#,
    )
    .bind(uid)
    .bind(uid)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Real accounts only, ordered by Vp desc, for the ranking list.
pub async fn top_by_vp(pool: &SqlitePool, limit: i32) -> sqlx::Result<Vec<ColosseumUserInfo>> {
    sqlx::query_as::<_, ColosseumUserInfo>("SELECT * FROM ColosseumUserInfo ORDER BY Vp DESC LIMIT ?")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn rank_of(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    let row: (i32,) = sqlx::query_as(
        r#"
SELECT 1 + COUNT(*) FROM ColosseumUserInfo
WHERE Vp > (SELECT Vp FROM ColosseumUserInfo WHERE Uid = ?)
"#,
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
