use crate::models::game::pvp::pvp_battle_history_info::PvpBattleHistoryInfo;
use sqlx::SqlitePool;

/// Add a single PvpBattleHistoryInfo record from a Rust struct.
pub async fn add_pvp_battle_history_info(
    pool: &SqlitePool,
    data: &PvpBattleHistoryInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpBattleHistoryInfo (
    Uid,
    BattleIndex,
    BattleResult,
    EnemyOwnerIndex,
    EnemyUserId,
    EnemyVp,
    EnemyRank,
    ChangeVp,
    ContinueWinVp,
    TimeValue,
    IsNoGame
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.battle_index)
    .bind(&data.battle_result)
    .bind(&data.enemy_owner_index)
    .bind(&data.enemy_user_id)
    .bind(&data.enemy_vp)
    .bind(&data.enemy_rank)
    .bind(&data.change_vp)
    .bind(&data.continue_win_vp)
    .bind(&data.time_value)
    .bind(&data.is_no_game)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pvp_battle_history_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PvpBattleHistoryInfo>> {
    sqlx::query_as::<_, PvpBattleHistoryInfo>("SELECT * FROM PvpBattleHistoryInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PvpBattleHistoryInfo rows for a UID.
pub async fn delete_pvp_battle_history_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpBattleHistoryInfo WHERE Uid = ?")
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
) -> sqlx::Result<PvpBattleHistoryInfo> {
    sqlx::query_as::<_, PvpBattleHistoryInfo>(
        "SELECT * FROM PvpBattleHistoryInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PvpBattleHistoryInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PvpBattleHistoryInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PvpBattleHistoryInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PvpBattleHistoryInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PvpBattleHistoryInfo (
    Uid,
    BattleIndex,
    BattleResult,
    EnemyOwnerIndex,
    EnemyUserId,
    EnemyVp,
    EnemyRank,
    ChangeVp,
    ContinueWinVp,
    TimeValue,
    IsNoGame
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.battle_index)
    .bind(&data.battle_result)
    .bind(&data.enemy_owner_index)
    .bind(&data.enemy_user_id)
    .bind(&data.enemy_vp)
    .bind(&data.enemy_rank)
    .bind(&data.change_vp)
    .bind(&data.continue_win_vp)
    .bind(&data.time_value)
    .bind(&data.is_no_game)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
