use crate::models::game::pvp::pvp_season_reward::PvpSeasonRewardClaim;
use sqlx::SqlitePool;

pub async fn is_claimed(pool: &SqlitePool, uid: i64, season: i32) -> sqlx::Result<bool> {
    let row: Option<PvpSeasonRewardClaim> =
        sqlx::query_as("SELECT * FROM PvpSeasonRewardClaim WHERE Uid = ? AND Season = ?")
            .bind(uid)
            .bind(season)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

pub async fn claim(pool: &SqlitePool, uid: i64, season: i32, now: i64) -> sqlx::Result<()> {
    sqlx::query("INSERT OR IGNORE INTO PvpSeasonRewardClaim (Uid, Season, ClaimedAt) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(season)
        .bind(now)
        .execute(pool)
        .await?;
    Ok(())
}
