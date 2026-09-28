use crate::models::game::pvp::pvp_user_info::PvpUserInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<PvpUserInfo>> {
    sqlx::query_as::<_, PvpUserInfo>("SELECT * FROM PvpUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<PvpUserInfo> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = PvpUserInfo { uid, ..Default::default() };
    insert(pool, &data).await?;
    Ok(data)
}

pub async fn insert(pool: &SqlitePool, data: &PvpUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpUserInfo (
    Uid, Vp, WinCount, LoseCount,
    SeasonAttackWinCount, SeasonAttackLoseCount, SeasonDefenseWinCount, SeasonDefenseLoseCount,
    PrevSeasonAttackWinCount, PrevSeasonAttackLoseCount, PrevSeasonDefenseWinCount, PrevSeasonDefenseLoseCount,
    DeckSeasonAttackWinCount, DeckSeasonAttackLoseCount, DeckSeasonDefenseWinCount, DeckSeasonDefenseLoseCount,
    DeckSeasonAttackResetTime, DeckSeasonDefenseResetTime,
    Season, CurrentBattleEnemyIndex, OnceRewardClaimed, SeasonRewardClaimedSeason
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.vp)
    .bind(data.win_count)
    .bind(data.lose_count)
    .bind(data.season_attack_win_count)
    .bind(data.season_attack_lose_count)
    .bind(data.season_defense_win_count)
    .bind(data.season_defense_lose_count)
    .bind(data.prev_season_attack_win_count)
    .bind(data.prev_season_attack_lose_count)
    .bind(data.prev_season_defense_win_count)
    .bind(data.prev_season_defense_lose_count)
    .bind(data.deck_season_attack_win_count)
    .bind(data.deck_season_attack_lose_count)
    .bind(data.deck_season_defense_win_count)
    .bind(data.deck_season_defense_lose_count)
    .bind(data.deck_season_attack_reset_time)
    .bind(data.deck_season_defense_reset_time)
    .bind(data.season)
    .bind(data.current_battle_enemy_index)
    .bind(&data.once_reward_claimed)
    .bind(data.season_reward_claimed_season)
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
    sqlx::query("UPDATE PvpUserInfo SET CurrentBattleEnemyIndex = ? WHERE Uid = ?")
        .bind(enemy_owner_index)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Applies a battle result as the attacker: adjusts Vp (clamped at 0), win/lose totals,
/// season attack win/lose, and deck-season attack win/lose.
pub async fn apply_attack_result(pool: &SqlitePool, uid: i64, vp_delta: i32, won: bool) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    if won {
        sqlx::query(
            "UPDATE PvpUserInfo SET Vp = MAX(0, Vp + ?), WinCount = WinCount + 1, \
             SeasonAttackWinCount = SeasonAttackWinCount + 1, DeckSeasonAttackWinCount = DeckSeasonAttackWinCount + 1 \
             WHERE Uid = ?",
        )
    } else {
        sqlx::query(
            "UPDATE PvpUserInfo SET Vp = MAX(0, Vp + ?), LoseCount = LoseCount + 1, \
             SeasonAttackLoseCount = SeasonAttackLoseCount + 1, DeckSeasonAttackLoseCount = DeckSeasonAttackLoseCount + 1 \
             WHERE Uid = ?",
        )
    }
    .bind(vp_delta)
    .bind(uid)
    .execute(pool)
    .await?;
    let row: (i32,) = sqlx::query_as("SELECT Vp FROM PvpUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

/// Applies a battle result as the defender (mirrored record for a real opponent).
pub async fn apply_defense_result(pool: &SqlitePool, uid: i64, vp_delta: i32, won: bool) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    if won {
        sqlx::query(
            "UPDATE PvpUserInfo SET Vp = MAX(0, Vp + ?), WinCount = WinCount + 1, \
             SeasonDefenseWinCount = SeasonDefenseWinCount + 1, DeckSeasonDefenseWinCount = DeckSeasonDefenseWinCount + 1 \
             WHERE Uid = ?",
        )
    } else {
        sqlx::query(
            "UPDATE PvpUserInfo SET Vp = MAX(0, Vp + ?), LoseCount = LoseCount + 1, \
             SeasonDefenseLoseCount = SeasonDefenseLoseCount + 1, DeckSeasonDefenseLoseCount = DeckSeasonDefenseLoseCount + 1 \
             WHERE Uid = ?",
        )
    }
    .bind(vp_delta)
    .bind(uid)
    .execute(pool)
    .await?;
    let row: (i32,) = sqlx::query_as("SELECT Vp FROM PvpUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn reset_deck_season(pool: &SqlitePool, uid: i64, deck_type: i32, now: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    if deck_type == 0 {
        sqlx::query(
            "UPDATE PvpUserInfo SET DeckSeasonAttackWinCount = 0, DeckSeasonAttackLoseCount = 0, DeckSeasonAttackResetTime = ? WHERE Uid = ?",
        )
    } else {
        sqlx::query(
            "UPDATE PvpUserInfo SET DeckSeasonDefenseWinCount = 0, DeckSeasonDefenseLoseCount = 0, DeckSeasonDefenseResetTime = ? WHERE Uid = ?",
        )
    }
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_once_reward_claimed(pool: &SqlitePool, uid: i64, claimed_csv: &str) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE PvpUserInfo SET OnceRewardClaimed = ? WHERE Uid = ?")
        .bind(claimed_csv)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_season_reward_claimed(pool: &SqlitePool, uid: i64, season: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE PvpUserInfo SET SeasonRewardClaimedSeason = ? WHERE Uid = ?")
        .bind(season)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Real accounts near the caller's Vp, excluding the caller.
pub async fn find_nearby_opponents(pool: &SqlitePool, uid: i64, limit: i32) -> sqlx::Result<Vec<PvpUserInfo>> {
    sqlx::query_as::<_, PvpUserInfo>(
        r#"
SELECT * FROM PvpUserInfo
WHERE Uid != ?
ORDER BY ABS(Vp - (SELECT Vp FROM PvpUserInfo WHERE Uid = ?))
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
pub async fn top_by_vp(pool: &SqlitePool, limit: i32) -> sqlx::Result<Vec<PvpUserInfo>> {
    sqlx::query_as::<_, PvpUserInfo>("SELECT * FROM PvpUserInfo ORDER BY Vp DESC LIMIT ?")
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn rank_of(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    let row: (i32,) = sqlx::query_as(
        "SELECT 1 + COUNT(*) FROM PvpUserInfo WHERE Vp > (SELECT Vp FROM PvpUserInfo WHERE Uid = ?)",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
