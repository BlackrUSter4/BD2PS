use crate::models::game::colosseum::colosseum_bless_info::ColosseumBlessInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ColosseumBlessInfo>> {
    sqlx::query_as::<_, ColosseumBlessInfo>("SELECT * FROM ColosseumBlessInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Replaces just one deck_type's (attack=0/defense=1, per the client's own numbering)
/// bless id list, inside a transaction — other deck types are left untouched.
pub async fn replace_for_type(
    pool: &SqlitePool,
    uid: i64,
    deck_type: i32,
    ids: &[i32],
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM ColosseumBlessInfo WHERE Uid = ? AND DeckType = ?")
        .bind(uid)
        .bind(deck_type)
        .execute(&mut *tx)
        .await?;
    for id in ids {
        sqlx::query("INSERT INTO ColosseumBlessInfo (Uid, DeckType, BlessId) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(deck_type)
            .bind(id)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
