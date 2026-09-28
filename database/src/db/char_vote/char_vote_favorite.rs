use crate::models::game::char_vote::char_vote_favorite::CharVoteFavorite;
use sqlx::SqlitePool;

pub async fn list(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CharVoteFavorite>> {
    sqlx::query_as::<_, CharVoteFavorite>("SELECT * FROM CharVoteFavorite WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn add(pool: &SqlitePool, uid: i64, candidate_id: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT OR IGNORE INTO CharVoteFavorite (Uid, CandidateId) VALUES (?, ?)")
        .bind(uid)
        .bind(candidate_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, uid: i64, candidate_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharVoteFavorite WHERE Uid = ? AND CandidateId = ?")
        .bind(uid)
        .bind(candidate_id)
        .execute(pool)
        .await?;
    Ok(())
}
