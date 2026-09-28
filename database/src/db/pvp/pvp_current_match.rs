use crate::models::game::pvp::pvp_current_match::PvpCurrentMatch;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<PvpCurrentMatch>> {
    sqlx::query_as::<_, PvpCurrentMatch>("SELECT * FROM PvpCurrentMatch WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn set(pool: &SqlitePool, data: &PvpCurrentMatch) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpCurrentMatch (Uid, EnemyOwnerIndex, EnemyUserId, EnemyVp, EnemyIsBot, EnemyCharIds, BattleRandomSeed, CreatedAt)
VALUES (?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    EnemyOwnerIndex = excluded.EnemyOwnerIndex,
    EnemyUserId = excluded.EnemyUserId,
    EnemyVp = excluded.EnemyVp,
    EnemyIsBot = excluded.EnemyIsBot,
    EnemyCharIds = excluded.EnemyCharIds,
    BattleRandomSeed = excluded.BattleRandomSeed,
    CreatedAt = excluded.CreatedAt
"#,
    )
    .bind(data.uid)
    .bind(data.enemy_owner_index)
    .bind(&data.enemy_user_id)
    .bind(data.enemy_vp)
    .bind(data.enemy_is_bot)
    .bind(&data.enemy_char_ids)
    .bind(data.battle_random_seed)
    .bind(data.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_seed(pool: &SqlitePool, uid: i64, seed: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE PvpCurrentMatch SET BattleRandomSeed = ? WHERE Uid = ?")
        .bind(seed)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn clear(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpCurrentMatch WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
