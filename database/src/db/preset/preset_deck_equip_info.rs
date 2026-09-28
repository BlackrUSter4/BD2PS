use crate::models::game::preset::preset_deck_equip_info::PresetDeckEquipInfo;
use sqlx::SqlitePool;

/// Fetch all equip rows belonging to a specific PresetDeckInfo row.
pub async fn get_by_preset_deck_info_index(
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
