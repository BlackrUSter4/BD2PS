use crate::models::game::spine_interaction::spine_interaction_reward_claim::SpineInteractionRewardClaim;
use sqlx::SqlitePool;

pub async fn list_for_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<SpineInteractionRewardClaim>> {
    sqlx::query_as::<_, SpineInteractionRewardClaim>(
        "SELECT * FROM SpineInteractionRewardClaim WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn is_claimed(
    pool: &SqlitePool,
    uid: i64,
    interaction_group_id: i32,
    group_id: i32,
    id: i32,
) -> sqlx::Result<bool> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT 1 FROM SpineInteractionRewardClaim WHERE Uid = ? AND InteractionGroupId = ? AND GroupId = ? AND Id = ?",
    )
    .bind(uid)
    .bind(interaction_group_id)
    .bind(group_id)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Returns true if this call newly claimed it (false if it was already claimed).
pub async fn claim(
    pool: &SqlitePool,
    uid: i64,
    interaction_group_id: i32,
    group_id: i32,
    id: i32,
) -> sqlx::Result<bool> {
    let result = sqlx::query(
        r#"
INSERT OR IGNORE INTO SpineInteractionRewardClaim (Uid, InteractionGroupId, GroupId, Id)
VALUES (?, ?, ?, ?)
"#,
    )
    .bind(uid)
    .bind(interaction_group_id)
    .bind(group_id)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}
