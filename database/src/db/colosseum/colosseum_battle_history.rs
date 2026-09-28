use crate::models::game::colosseum::colosseum_battle_history::ColosseumBattleHistory;
use sqlx::SqlitePool;

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &SqlitePool,
    uid: i64,
    season: i32,
    is_attacker: bool,
    enemy_owner_index: Option<i64>,
    enemy_user_id: Option<&str>,
    enemy_vp: Option<i32>,
    enemy_rank: Option<i32>,
    change_vp: Option<i32>,
    time_value: i64,
    enemy_top_percent: Option<f64>,
    is_no_game: bool,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ColosseumBattleHistory (
    Uid, Season, IsAttacker, EnemyOwnerIndex, EnemyUserId, EnemyVp, EnemyRank,
    ChangeVp, TimeValue, EnemyTopPercent, IsNoGame
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(uid)
    .bind(season)
    .bind(is_attacker)
    .bind(enemy_owner_index)
    .bind(enemy_user_id)
    .bind(enemy_vp)
    .bind(enemy_rank)
    .bind(change_vp)
    .bind(time_value)
    .bind(enemy_top_percent)
    .bind(is_no_game)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn recent_by_uid(
    pool: &SqlitePool,
    uid: i64,
    is_attacker: bool,
    limit: i32,
) -> sqlx::Result<Vec<ColosseumBattleHistory>> {
    sqlx::query_as::<_, ColosseumBattleHistory>(
        "SELECT * FROM ColosseumBattleHistory WHERE Uid = ? AND IsAttacker = ? ORDER BY \"Index\" DESC LIMIT ?",
    )
    .bind(uid)
    .bind(is_attacker)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// (win_count, lose_count) for a given uid/season/attacker-or-defender, ignoring no-game rows.
pub async fn win_lose_count(
    pool: &SqlitePool,
    uid: i64,
    season: i32,
    is_attacker: bool,
) -> sqlx::Result<(i32, i32)> {
    let wins: (i32,) = sqlx::query_as(
        "SELECT COUNT(*) FROM ColosseumBattleHistory WHERE Uid = ? AND Season = ? AND IsAttacker = ? AND IsNoGame = 0 AND ChangeVp > 0",
    )
    .bind(uid)
    .bind(season)
    .bind(is_attacker)
    .fetch_one(pool)
    .await?;
    let losses: (i32,) = sqlx::query_as(
        "SELECT COUNT(*) FROM ColosseumBattleHistory WHERE Uid = ? AND Season = ? AND IsAttacker = ? AND IsNoGame = 0 AND ChangeVp <= 0",
    )
    .bind(uid)
    .bind(season)
    .bind(is_attacker)
    .fetch_one(pool)
    .await?;
    Ok((wins.0, losses.0))
}
