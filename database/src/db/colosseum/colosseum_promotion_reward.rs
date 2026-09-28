use crate::models::game::colosseum::colosseum_promotion_reward::ColosseumPromotionReward;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ColosseumPromotionReward>> {
    sqlx::query_as::<_, ColosseumPromotionReward>(
        "SELECT * FROM ColosseumPromotionReward WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn has_reward(pool: &SqlitePool, uid: i64, reward_id: i32) -> sqlx::Result<bool> {
    let row: (i32,) = sqlx::query_as(
        "SELECT COUNT(*) FROM ColosseumPromotionReward WHERE Uid = ? AND RewardId = ?",
    )
    .bind(uid)
    .bind(reward_id)
    .fetch_one(pool)
    .await?;
    Ok(row.0 > 0)
}

pub async fn grant(pool: &SqlitePool, uid: i64, reward_id: i32) -> sqlx::Result<()> {
    if !has_reward(pool, uid, reward_id).await? {
        sqlx::query("INSERT INTO ColosseumPromotionReward (Uid, RewardId) VALUES (?, ?)")
            .bind(uid)
            .bind(reward_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
