use crate::models::game::colosseum::colosseum_deck_info::ColosseumDeckInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ColosseumDeckInfo>> {
    sqlx::query_as::<_, ColosseumDeckInfo>(
        "SELECT * FROM ColosseumDeckInfo WHERE Uid = ? ORDER BY Position",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub struct DeckSlot {
    pub position: i32,
    pub char_inven_index: i64,
    pub sequence: Option<i32>,
    pub costume_inven_index: Option<i64>,
}

/// Replaces the caller's entire deck (delete + reinsert), inside a transaction.
pub async fn replace_all(pool: &SqlitePool, uid: i64, slots: &[DeckSlot]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM ColosseumDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for slot in slots {
        sqlx::query(
            "INSERT INTO ColosseumDeckInfo (Uid, Position, CharInvenIndex, Sequence, CostumeInvenIndex) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(slot.position)
        .bind(slot.char_inven_index)
        .bind(slot.sequence)
        .bind(slot.costume_inven_index)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
