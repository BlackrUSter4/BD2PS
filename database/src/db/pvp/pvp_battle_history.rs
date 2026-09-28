use crate::models::game::pvp::pvp_battle_history::PvpBattleHistory;
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
    continue_win_vp: Option<i32>,
    time_value: i64,
    is_no_game: bool,
    seed: Option<i32>,
    deck_snapshot_json: Option<&str>,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleHistory (
    Uid, Season, IsAttacker, EnemyOwnerIndex, EnemyUserId, EnemyVp, EnemyRank,
    ChangeVp, ContinueWinVp, TimeValue, IsNoGame, Seed, DeckSnapshotJson
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
    .bind(continue_win_vp)
    .bind(time_value)
    .bind(is_no_game)
    .bind(seed)
    .bind(deck_snapshot_json)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn list_attack(pool: &SqlitePool, uid: i64, limit: i32) -> sqlx::Result<Vec<PvpBattleHistory>> {
    sqlx::query_as::<_, PvpBattleHistory>(
        "SELECT * FROM PvpBattleHistory WHERE Uid = ? AND IsAttacker = 1 ORDER BY BattleIndex DESC LIMIT ?",
    )
    .bind(uid)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn list_defense(pool: &SqlitePool, uid: i64, limit: i32) -> sqlx::Result<Vec<PvpBattleHistory>> {
    sqlx::query_as::<_, PvpBattleHistory>(
        "SELECT * FROM PvpBattleHistory WHERE Uid = ? AND IsAttacker = 0 ORDER BY BattleIndex DESC LIMIT ?",
    )
    .bind(uid)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_by_index(pool: &SqlitePool, uid: i64, battle_index: i64) -> sqlx::Result<Option<PvpBattleHistory>> {
    sqlx::query_as::<_, PvpBattleHistory>("SELECT * FROM PvpBattleHistory WHERE Uid = ? AND BattleIndex = ?")
        .bind(uid)
        .bind(battle_index)
        .fetch_optional(pool)
        .await
}
