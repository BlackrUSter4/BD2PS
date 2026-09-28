use crate::models::game::preset::preset_deck_info::PresetDeckInfo;
use sqlx::SqlitePool;

/// Fetch all deck-slot rows belonging to a specific PresetInfo row.
pub async fn get_by_preset_info_index(
    pool: &SqlitePool,
    preset_info_index: i64,
) -> sqlx::Result<Vec<PresetDeckInfo>> {
    sqlx::query_as::<_, PresetDeckInfo>("SELECT * FROM PresetDeckInfo WHERE PresetInfoIndex = ?")
        .bind(preset_info_index)
        .fetch_all(pool)
        .await
}
