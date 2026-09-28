use crate::models::game::colosseum::colosseum_season_reward::ColosseumSeasonReward;
use sqlx::SqlitePool;

pub async fn get(
    pool: &SqlitePool,
    uid: i64,
    season: i32,
) -> sqlx::Result<Option<ColosseumSeasonReward>> {
    sqlx::query_as::<_, ColosseumSeasonReward>(
        "SELECT * FROM ColosseumSeasonReward WHERE Uid = ? AND Season = ?",
    )
    .bind(uid)
    .bind(season)
    .fetch_optional(pool)
    .await
}

pub async fn is_claimed(pool: &SqlitePool, uid: i64, season: i32) -> sqlx::Result<bool> {
    Ok(get(pool, uid, season).await?.map(|r| r.is_claimed).unwrap_or(false))
}

pub async fn claim(pool: &SqlitePool, uid: i64, season: i32) -> sqlx::Result<()> {
    if get(pool, uid, season).await?.is_some() {
        sqlx::query("UPDATE ColosseumSeasonReward SET IsClaimed = 1 WHERE Uid = ? AND Season = ?")
            .bind(uid)
            .bind(season)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO ColosseumSeasonReward (Uid, Season, IsClaimed) VALUES (?, ?, 1)")
            .bind(uid)
            .bind(season)
            .execute(pool)
            .await?;
    }
    Ok(())
}
