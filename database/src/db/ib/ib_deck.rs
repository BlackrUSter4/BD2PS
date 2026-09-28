use crate::models::game::ib::ib_deck::IbDeck;
use sqlx::SqlitePool;

pub struct DeckSlot {
    pub position: i32,
    pub inven_index: i64,
    pub rotation_count: i32,
}

pub async fn list(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<IbDeck>> {
    sqlx::query_as::<_, IbDeck>("SELECT * FROM IbDeck WHERE Uid = ? ORDER BY Position")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn replace_all(pool: &SqlitePool, uid: i64, slots: &[DeckSlot]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM IbDeck WHERE Uid = ?").bind(uid).execute(&mut *tx).await?;
    for slot in slots {
        sqlx::query("INSERT INTO IbDeck (Uid, Position, InvenIndex, RotationCount) VALUES (?, ?, ?, ?)")
            .bind(uid)
            .bind(slot.position)
            .bind(slot.inven_index)
            .bind(slot.rotation_count)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(())
}
