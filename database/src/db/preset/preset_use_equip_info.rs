use crate::models::game::preset::preset_use_equip_info::PresetUseEquipInfo;
use sqlx::SqlitePool;

/// Fetch all records for a given UID.
pub async fn get_preset_use_equip_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PresetUseEquipInfo>> {
    sqlx::query_as::<_, PresetUseEquipInfo>("SELECT * FROM PresetUseEquipInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Replace the recorded "last preset use" equip snapshot for the whole account
/// (delete + reinsert one row per (char, equip) pair), inside a transaction.
pub async fn replace_for_uid(
    pool: &SqlitePool,
    uid: i64,
    chars: &[(i64, Vec<i64>)],
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM PresetUseEquipInfo WHERE Uid = ?")
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    for (char_inven_index, equip_inven_index) in chars {
        for equip in equip_inven_index {
            sqlx::query(
                "INSERT INTO PresetUseEquipInfo (Uid, CharInvenIndex, EquipInvenIndex) VALUES (?, ?, ?)",
            )
            .bind(uid)
            .bind(char_inven_index)
            .bind(equip)
            .execute(&mut *tx)
            .await?;
        }
    }
    tx.commit().await?;
    Ok(())
}

/// Delete all PresetUseEquipInfo rows for a UID.
pub async fn delete_preset_use_equip_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PresetUseEquipInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
