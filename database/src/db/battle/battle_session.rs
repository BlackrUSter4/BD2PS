use crate::models::game::battle::battle_session::BattleSession;
use sqlx::SqlitePool;

/// Insert or replace the caller's single in-progress battle session.
pub async fn upsert(pool: &SqlitePool, data: &BattleSession) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO BattleSession (
    Uid, BattleIndex, GroupId, MonsterId, PackId, BattleDeck, BattleMode,
    MonsterHuntId, StageMagicGroupId, StageMagicId, StageMagicLevel, RandomSeed, CreatedAt
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    BattleIndex = excluded.BattleIndex,
    GroupId = excluded.GroupId,
    MonsterId = excluded.MonsterId,
    PackId = excluded.PackId,
    BattleDeck = excluded.BattleDeck,
    BattleMode = excluded.BattleMode,
    MonsterHuntId = excluded.MonsterHuntId,
    StageMagicGroupId = excluded.StageMagicGroupId,
    StageMagicId = excluded.StageMagicId,
    StageMagicLevel = excluded.StageMagicLevel,
    RandomSeed = excluded.RandomSeed,
    CreatedAt = excluded.CreatedAt
"#,
    )
    .bind(data.uid)
    .bind(data.battle_index)
    .bind(data.group_id)
    .bind(data.monster_id)
    .bind(data.pack_id)
    .bind(data.battle_deck)
    .bind(data.battle_mode)
    .bind(data.monster_hunt_id)
    .bind(data.stage_magic_group_id)
    .bind(data.stage_magic_id)
    .bind(data.stage_magic_level)
    .bind(data.random_seed)
    .bind(data.created_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<BattleSession>> {
    sqlx::query_as::<_, BattleSession>("SELECT * FROM BattleSession WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Set just the random seed + battle index for the Start step, keeping the
/// rest of the session (set at Enter) intact.
pub async fn set_start(pool: &SqlitePool, uid: i64, battle_index: i32, seed: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE BattleSession SET BattleIndex = ?, RandomSeed = ? WHERE Uid = ?")
        .bind(battle_index)
        .bind(seed)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM BattleSession WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
