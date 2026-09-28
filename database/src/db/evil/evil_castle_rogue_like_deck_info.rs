use crate::models::game::evil::evil_castle_rogue_like_deck_info::EvilCastleRogueLikeDeckInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EvilCastleRogueLikeDeckInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeDeckInfo>(
        "SELECT * FROM EvilCastleRogueLikeDeckInfo WHERE Uid = ? ORDER BY Position",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn delete(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeDeckInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert(pool: &SqlitePool, uid: i64, char_inven_index: i64, position: i32, sequence: i32) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO EvilCastleRogueLikeDeckInfo (Uid, CharInvenIndex, Position, Sequence) VALUES (?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(char_inven_index)
    .bind(position)
    .bind(sequence)
    .execute(pool)
    .await?;
    Ok(())
}

/// Replace the caller's whole roguelike deck (delete-then-insert).
pub async fn save(pool: &SqlitePool, uid: i64, entries: &[(i64, i32, i32)]) -> sqlx::Result<()> {
    delete(pool, uid).await?;
    for (char_inven_index, position, sequence) in entries {
        insert(pool, uid, *char_inven_index, *position, *sequence).await?;
    }
    Ok(())
}
