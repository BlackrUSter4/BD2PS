use crate::models::game::equip::equip_preset_info::EquipPresetInfo;
use sqlx::SqlitePool;

/// Add a single EquipPresetInfo record from a Rust struct.
pub async fn add_equip_preset_info(pool: &SqlitePool, data: &EquipPresetInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EquipPresetInfo (
    Uid,
    PresetName,
    Slot,
    PresetResourceId,
    PresetResourceColor
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.preset_name)
    .bind(&data.slot)
    .bind(&data.preset_resource_id)
    .bind(&data.preset_resource_color)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_equip_preset_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EquipPresetInfo>> {
    sqlx::query_as::<_, EquipPresetInfo>("SELECT * FROM EquipPresetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EquipPresetInfo rows for a UID.
pub async fn delete_equip_preset_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EquipPresetInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<EquipPresetInfo> {
    sqlx::query_as::<_, EquipPresetInfo>(
        "SELECT * FROM EquipPresetInfo WHERE Uid = ? AND Index = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<EquipPresetInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EquipPresetInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EquipPresetInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EquipPresetInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EquipPresetInfo (
    Uid,
    PresetName,
    Slot,
    PresetResourceId,
    PresetResourceColor,
    ItemInfoIndex
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.preset_name)
    .bind(&data.slot)
    .bind(&data.preset_resource_id)
    .bind(&data.preset_resource_color)
    .bind(&data.item_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update(pool: &SqlitePool, uid: i64, index: i64, preset_name: Option<&str>, preset_resource_id: Option<i32>, preset_resource_color: Option<i32>, item_info_index: Option<&str>) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EquipPresetInfo SET PresetName = COALESCE(?, PresetName), PresetResourceId = COALESCE(?, PresetResourceId), PresetResourceColor = COALESCE(?, PresetResourceColor), ItemInfoIndex = COALESCE(?, ItemInfoIndex) WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(preset_name)
    .bind(preset_resource_id)
    .bind(preset_resource_color)
    .bind(item_info_index)
    .bind(uid)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}
