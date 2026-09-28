use crate::models::game::my::my_room_preset_info::MyRoomPresetInfo;
use sqlx::SqlitePool;

/// Fetch every preset of a given type for an account.
pub async fn get_all(pool: &SqlitePool, uid: i64, preset_type: i32) -> sqlx::Result<Vec<MyRoomPresetInfo>> {
    sqlx::query_as::<_, MyRoomPresetInfo>(
        "SELECT * FROM MyRoomPresetInfo WHERE Uid = ? AND PresetType = ?",
    )
    .bind(uid)
    .bind(preset_type)
    .fetch_all(pool)
    .await
}

/// Fetch a single preset slot.
pub async fn get_one(
    pool: &SqlitePool,
    uid: i64,
    preset_type: i32,
    slot: i32,
) -> sqlx::Result<Option<MyRoomPresetInfo>> {
    sqlx::query_as::<_, MyRoomPresetInfo>(
        "SELECT * FROM MyRoomPresetInfo WHERE Uid = ? AND PresetType = ? AND Slot = ?",
    )
    .bind(uid)
    .bind(preset_type)
    .bind(slot)
    .fetch_optional(pool)
    .await
}

/// Create or overwrite a preset slot.
#[allow(clippy::too_many_arguments)]
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    preset_type: i32,
    slot: i32,
    name: Option<&str>,
    source_owner_index: Option<i64>,
    item_info_json: &str,
    room_info_json: &str,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomPresetInfo (Uid, PresetType, Slot, Name, SourceOwnerIndex, ItemInfoJson, RoomInfoJson)
VALUES (?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid, PresetType, Slot) DO UPDATE SET
    Name = excluded.Name,
    SourceOwnerIndex = excluded.SourceOwnerIndex,
    ItemInfoJson = excluded.ItemInfoJson,
    RoomInfoJson = excluded.RoomInfoJson
"#,
    )
    .bind(uid)
    .bind(preset_type)
    .bind(slot)
    .bind(name)
    .bind(source_owner_index)
    .bind(item_info_json)
    .bind(room_info_json)
    .execute(pool)
    .await?;
    Ok(())
}

/// Rename a preset slot (no-op if it doesn't exist yet).
pub async fn update_name(
    pool: &SqlitePool,
    uid: i64,
    preset_type: i32,
    slot: i32,
    name: &str,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE MyRoomPresetInfo SET Name = ? WHERE Uid = ? AND PresetType = ? AND Slot = ?")
        .bind(name)
        .bind(uid)
        .bind(preset_type)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_one(pool: &SqlitePool, uid: i64, preset_type: i32, slot: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomPresetInfo WHERE Uid = ? AND PresetType = ? AND Slot = ?")
        .bind(uid)
        .bind(preset_type)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}
