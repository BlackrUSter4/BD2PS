use crate::models::game::colosseum::colosseum_match_candidate::ColosseumMatchCandidate;
use sqlx::SqlitePool;

pub struct Candidate {
    pub enemy_owner_index: i64,
    pub enemy_user_id: Option<String>,
    pub enemy_vp: Option<i32>,
    pub enemy_is_bot: bool,
    pub enemy_char_ids: Option<String>,
}

/// Replaces the caller's cached match candidates (from the most recent matching request) —
/// looked up again by BattleStart via `enemy_owner_index`, since bot opponents aren't real
/// accounts that could otherwise be looked up.
pub async fn replace_all(
    pool: &SqlitePool,
    uid: i64,
    candidates: &[Candidate],
    now_ms: i64,
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM ColosseumMatchCandidate WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for c in candidates {
        sqlx::query(
            r#"
INSERT INTO ColosseumMatchCandidate (
    Uid, EnemyOwnerIndex, EnemyUserId, EnemyVp, EnemyIsBot, EnemyCharIds, CreateTime
) VALUES (?, ?, ?, ?, ?, ?, ?)
"#,
        )
        .bind(uid)
        .bind(c.enemy_owner_index)
        .bind(&c.enemy_user_id)
        .bind(c.enemy_vp)
        .bind(c.enemy_is_bot)
        .bind(&c.enemy_char_ids)
        .bind(now_ms)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn find(
    pool: &SqlitePool,
    uid: i64,
    enemy_owner_index: i64,
) -> sqlx::Result<Option<ColosseumMatchCandidate>> {
    sqlx::query_as::<_, ColosseumMatchCandidate>(
        "SELECT * FROM ColosseumMatchCandidate WHERE Uid = ? AND EnemyOwnerIndex = ?",
    )
    .bind(uid)
    .bind(enemy_owner_index)
    .fetch_optional(pool)
    .await
}
