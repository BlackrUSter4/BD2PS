use crate::models::game::equip::equip_preset_char_info::EquipPresetCharInfo;
use sqlx::SqlitePool;

/// Add a single EquipPresetCharInfo record from a Rust struct.
pub async fn add_equip_preset_char_info(
    pool: &SqlitePool,
    data: &EquipPresetCharInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EquipPresetCharInfo (
    Uid,
    CharInvenIndex,
    PresetInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.preset_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_equip_preset_char_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EquipPresetCharInfo>> {
    sqlx::query_as::<_, EquipPresetCharInfo>("SELECT * FROM EquipPresetCharInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EquipPresetCharInfo rows for a UID.
pub async fn delete_equip_preset_char_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EquipPresetCharInfo WHERE Uid = ?")
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
) -> sqlx::Result<EquipPresetCharInfo> {
    sqlx::query_as::<_, EquipPresetCharInfo>(
        "SELECT * FROM EquipPresetCharInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EquipPresetCharInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EquipPresetCharInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EquipPresetCharInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EquipPresetCharInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EquipPresetCharInfo (
    Uid,
    CharInvenIndex,
    PresetInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.char_inven_index)
    .bind(&data.preset_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
