use crate::models::game::monster::monster_hunt_preset_info::MonsterHuntPresetInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MonsterHuntPresetInfo records for a UID.
pub async fn insert_monster_hunt_preset_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("monsterHuntPresetInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_monster_hunt_preset_info: missing or invalid 'monsterHuntPresetInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested PresetInfo - extract InvenIndex values
        let preset_info_index =
            if let Some(nested_arr) = entry.get("presetInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };

        sqlx::query(
            r#"
INSERT INTO MonsterHuntPresetInfo (
    Uid,
    PresetInfoIndex
) VALUES (
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&preset_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MonsterHuntPresetInfo record from a Rust struct.
pub async fn add_monster_hunt_preset_info(
    pool: &SqlitePool,
    data: &MonsterHuntPresetInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntPresetInfo (
    Uid,
    PresetInfoIndex
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.preset_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_preset_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntPresetInfo>> {
    sqlx::query_as::<_, MonsterHuntPresetInfo>("SELECT * FROM MonsterHuntPresetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterHuntPresetInfo rows for a UID.
pub async fn delete_monster_hunt_preset_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntPresetInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get the preset row for one addressable slot.
pub async fn get_by_uid_and_slot(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
) -> sqlx::Result<Option<MonsterHuntPresetInfo>> {
    sqlx::query_as::<_, MonsterHuntPresetInfo>(
        "SELECT * FROM MonsterHuntPresetInfo WHERE Uid = ? AND Slot = ?",
    )
    .bind(uid)
    .bind(slot)
    .fetch_optional(pool)
    .await
}

/// Highest slot number currently used by this account (for adding new slots).
pub async fn max_slot(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    let row: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(Slot) FROM MonsterHuntPresetInfo WHERE Uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(row.0.unwrap_or(0))
}

/// Create a new empty preset slot, returning its slot number.
pub async fn add_slot(pool: &SqlitePool, uid: i64, slot: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO MonsterHuntPresetInfo (Uid, Slot) VALUES (?, ?)")
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}

/// Save/overwrite a preset's name/resource/deck contents for one slot.
#[allow(clippy::too_many_arguments)]
pub async fn save_slot(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
    preset_name: Option<&str>,
    preset_resource_id: Option<i32>,
    preset_resource_color: Option<i32>,
    preset_info_index: Option<&str>,
) -> sqlx::Result<()> {
    if get_by_uid_and_slot(pool, uid, slot).await?.is_some() {
        sqlx::query(
            r#"
UPDATE MonsterHuntPresetInfo SET
    PresetName = COALESCE(?, PresetName),
    PresetResourceId = COALESCE(?, PresetResourceId),
    PresetResourceColor = COALESCE(?, PresetResourceColor),
    PresetInfoIndex = COALESCE(?, PresetInfoIndex)
WHERE Uid = ? AND Slot = ?
"#,
        )
        .bind(preset_name)
        .bind(preset_resource_id)
        .bind(preset_resource_color)
        .bind(preset_info_index)
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
INSERT INTO MonsterHuntPresetInfo (Uid, Slot, PresetName, PresetResourceId, PresetResourceColor, PresetInfoIndex)
VALUES (?, ?, ?, ?, ?, ?)
"#,
        )
        .bind(uid)
        .bind(slot)
        .bind(preset_name)
        .bind(preset_resource_id)
        .bind(preset_resource_color)
        .bind(preset_info_index)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Delete a set of slots for this account.
pub async fn delete_slots(pool: &SqlitePool, uid: i64, slots: &[i32]) -> sqlx::Result<()> {
    if slots.is_empty() {
        return Ok(());
    }
    let placeholders = slots.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "DELETE FROM MonsterHuntPresetInfo WHERE Uid = ? AND Slot IN ({})",
        placeholders
    );
    let mut q = sqlx::query(&query).bind(uid);
    for s in slots {
        q = q.bind(s);
    }
    q.execute(pool).await?;
    Ok(())
}
