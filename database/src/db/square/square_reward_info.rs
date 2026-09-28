use crate::models::game::square::square_reward_info::SquareRewardInfo;
use sqlx::SqlitePool;

pub async fn get_or_default(pool: &SqlitePool, uid: i64) -> SquareRewardInfo {
    sqlx::query_as::<_, SquareRewardInfo>("SELECT * FROM SquareRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(SquareRewardInfo { uid, last_claim_date: None })
}

pub async fn set_claimed(pool: &SqlitePool, uid: i64, date: &str) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SquareRewardInfo (Uid, LastClaimDate) VALUES (?, ?)
ON CONFLICT(Uid) DO UPDATE SET LastClaimDate = excluded.LastClaimDate
"#,
    )
    .bind(uid)
    .bind(date)
    .execute(pool)
    .await?;
    Ok(())
}
