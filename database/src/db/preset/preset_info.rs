use crate::models::game::preset::preset_deck_equip_info::PresetDeckEquipInfo;
use crate::models::game::preset::preset_deck_info::PresetDeckInfo;
use crate::models::game::preset::preset_info::PresetInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PresetInfo>> {
    sqlx::query_as::<_, PresetInfo>("SELECT * FROM PresetInfo WHERE Uid = ? ORDER BY Slot")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_uid_and_slot(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
) -> sqlx::Result<Option<PresetInfo>> {
    sqlx::query_as::<_, PresetInfo>("SELECT * FROM PresetInfo WHERE Uid = ? AND Slot = ?")
        .bind(uid)
        .bind(slot)
        .fetch_optional(pool)
        .await
}

pub async fn max_slot(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    let row: (Option<i32>,) = sqlx::query_as("SELECT MAX(Slot) FROM PresetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0.unwrap_or(-1))
}

/// Inserts a new preset shell, returning its autoincrement Index.
pub async fn insert(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
    name: Option<&str>,
    resource_id: Option<i32>,
    resource_color: Option<i32>,
) -> sqlx::Result<i64> {
    let result = sqlx::query(
        "INSERT INTO PresetInfo (Uid, Slot, PresetName, PresetResourceId, PresetResourceColor) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(slot)
    .bind(name)
    .bind(resource_id)
    .bind(resource_color)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn update_info(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
    name: Option<&str>,
    resource_id: Option<i32>,
    resource_color: Option<i32>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE PresetInfo SET PresetName = ?, PresetResourceId = ?, PresetResourceColor = ? WHERE Uid = ? AND Slot = ?",
    )
    .bind(name)
    .bind(resource_id)
    .bind(resource_color)
    .bind(uid)
    .bind(slot)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_by_slots(pool: &SqlitePool, uid: i64, slots: &[i32]) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    for slot in slots {
        if let Some(preset) =
            sqlx::query_as::<_, PresetInfo>("SELECT * FROM PresetInfo WHERE Uid = ? AND Slot = ?")
                .bind(uid)
                .bind(slot)
                .fetch_optional(&mut *tx)
                .await?
        {
            let deck_indices: Vec<i64> =
                sqlx::query_scalar("SELECT \"Index\" FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
                    .bind(preset.index)
                    .fetch_all(&mut *tx)
                    .await?;
            for idx in deck_indices {
                sqlx::query("DELETE FROM PresetDeckEquipInfo WHERE PresetDeckInfoIndex = ?")
                    .bind(idx)
                    .execute(&mut *tx)
                    .await?;
            }
            sqlx::query("DELETE FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
                .bind(preset.index)
                .execute(&mut *tx)
                .await?;
            sqlx::query("DELETE FROM PresetInfo WHERE \"Index\" = ?")
                .bind(preset.index)
                .execute(&mut *tx)
                .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

pub struct PresetDeckSlot {
    pub char_inven_index: i64,
    pub position: Option<i32>,
    pub sequence: Option<i32>,
    pub costume_inven_index: Option<i64>,
    pub team: Option<i32>,
    pub equips: Vec<(Option<i32>, Option<i64>)>,
}

/// Replaces a preset's saved deck (delete + reinsert), inside a transaction.
pub async fn replace_deck(
    pool: &SqlitePool,
    preset_info_index: i64,
    slots: &[PresetDeckSlot],
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    let deck_indices: Vec<i64> =
        sqlx::query_scalar("SELECT \"Index\" FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
            .bind(preset_info_index)
            .fetch_all(&mut *tx)
            .await?;
    for idx in deck_indices {
        sqlx::query("DELETE FROM PresetDeckEquipInfo WHERE PresetDeckInfoIndex = ?")
            .bind(idx)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query("DELETE FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
        .bind(preset_info_index)
        .execute(&mut *tx)
        .await?;
    for slot in slots {
        let result = sqlx::query(
            "INSERT INTO PresetDeckInfo (PresetInfoIndex, CharInvenIndex, Position, Sequence, CostumeInvenIndex, Team) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(preset_info_index)
        .bind(slot.char_inven_index)
        .bind(slot.position)
        .bind(slot.sequence)
        .bind(slot.costume_inven_index)
        .bind(slot.team)
        .execute(&mut *tx)
        .await?;
        let deck_index = result.last_insert_rowid();
        for (equip_type, equip_inven_index) in &slot.equips {
            sqlx::query(
                "INSERT INTO PresetDeckEquipInfo (PresetDeckInfoIndex, EquipType, EquipInvenIndex) VALUES (?, ?, ?)",
            )
            .bind(deck_index)
            .bind(equip_type)
            .bind(equip_inven_index)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

pub async fn get_deck(
    pool: &SqlitePool,
    preset_info_index: i64,
) -> sqlx::Result<Vec<PresetDeckInfo>> {
    sqlx::query_as::<_, PresetDeckInfo>("SELECT * FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
        .bind(preset_info_index)
        .fetch_all(pool)
        .await
}

pub async fn get_deck_equips(
    pool: &SqlitePool,
    preset_deck_info_index: i64,
) -> sqlx::Result<Vec<PresetDeckEquipInfo>> {
    sqlx::query_as::<_, PresetDeckEquipInfo>(
        "SELECT * FROM PresetDeckEquipInfo WHERE PresetDeckInfoIndex = ?",
    )
    .bind(preset_deck_info_index)
    .fetch_all(pool)
    .await
}
