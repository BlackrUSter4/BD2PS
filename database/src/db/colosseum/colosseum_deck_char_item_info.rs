use crate::models::game::colosseum::colosseum_deck_char_equip_info::ColosseumDeckCharEquipInfo;
use crate::models::game::colosseum::colosseum_deck_char_item_info::ColosseumDeckCharItemInfo;
use sqlx::SqlitePool;

pub async fn get_items_by_uid(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ColosseumDeckCharItemInfo>> {
    sqlx::query_as::<_, ColosseumDeckCharItemInfo>(
        "SELECT * FROM ColosseumDeckCharItemInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn get_equips_by_uid(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ColosseumDeckCharEquipInfo>> {
    sqlx::query_as::<_, ColosseumDeckCharEquipInfo>(
        "SELECT * FROM ColosseumDeckCharEquipInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub struct CharItemSlot {
    pub char_inven_index: i64,
    pub connect_potential_costume: Option<i32>,
    pub equips: Vec<(Option<i32>, Option<i64>)>, // (equip_type, equip_inven_index)
}

/// Replaces the caller's whole deck-item loadout (delete + reinsert), inside a transaction.
pub async fn replace_all(pool: &SqlitePool, uid: i64, items: &[CharItemSlot]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM ColosseumDeckCharItemInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM ColosseumDeckCharEquipInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for item in items {
        sqlx::query(
            "INSERT INTO ColosseumDeckCharItemInfo (Uid, CharInvenIndex, ConnectPotentialCostume) VALUES (?, ?, ?)",
        )
        .bind(uid)
        .bind(item.char_inven_index)
        .bind(item.connect_potential_costume)
        .execute(&mut *tx)
        .await?;
        for (equip_type, equip_inven_index) in &item.equips {
            sqlx::query(
                "INSERT INTO ColosseumDeckCharEquipInfo (Uid, CharInvenIndex, EquipType, EquipInvenIndex) VALUES (?, ?, ?, ?)",
            )
            .bind(uid)
            .bind(item.char_inven_index)
            .bind(equip_type)
            .bind(equip_inven_index)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}
