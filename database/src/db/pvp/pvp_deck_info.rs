use crate::models::game::pvp::pvp_deck_info::{PvpDeckInfo, PvpDeckMeta};
use sqlx::SqlitePool;

pub struct DeckSlot {
    pub position: i32,
    pub char_inven_index: i64,
    pub sequence: Option<i32>,
    pub costume_inven_index: Option<i64>,
}

pub async fn get_by_uid_type(pool: &SqlitePool, uid: i64, deck_type: i32) -> sqlx::Result<Vec<PvpDeckInfo>> {
    sqlx::query_as::<_, PvpDeckInfo>(
        "SELECT * FROM PvpDeckInfo WHERE Uid = ? AND DeckType = ? ORDER BY Position ASC",
    )
    .bind(uid)
    .bind(deck_type)
    .fetch_all(pool)
    .await
}

pub async fn replace_all(pool: &SqlitePool, uid: i64, deck_type: i32, slots: &[DeckSlot]) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PvpDeckInfo WHERE Uid = ? AND DeckType = ?")
        .bind(uid)
        .bind(deck_type)
        .execute(pool)
        .await?;
    for slot in slots {
        sqlx::query(
            "INSERT INTO PvpDeckInfo (Uid, DeckType, CharInvenIndex, Position, Sequence, CostumeInvenIndex) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(uid)
        .bind(deck_type)
        .bind(slot.char_inven_index)
        .bind(slot.position)
        .bind(slot.sequence)
        .bind(slot.costume_inven_index)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_meta(pool: &SqlitePool, uid: i64, deck_type: i32) -> sqlx::Result<Option<PvpDeckMeta>> {
    sqlx::query_as::<_, PvpDeckMeta>("SELECT * FROM PvpDeckMeta WHERE Uid = ? AND DeckType = ?")
        .bind(uid)
        .bind(deck_type)
        .fetch_optional(pool)
        .await
}

pub async fn set_meta(
    pool: &SqlitePool,
    uid: i64,
    deck_type: i32,
    item_info_json: &str,
    battle_power: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PvpDeckMeta (Uid, DeckType, ItemInfoJson, BattlePower) VALUES (?, ?, ?, ?)
ON CONFLICT(Uid, DeckType) DO UPDATE SET ItemInfoJson = excluded.ItemInfoJson, BattlePower = excluded.BattlePower
"#,
    )
    .bind(uid)
    .bind(deck_type)
    .bind(item_info_json)
    .bind(battle_power)
    .execute(pool)
    .await?;
    Ok(())
}
