use crate::models::game::total::total_war_deck_preset_info::TotalWarDeckPresetInfo;
use sqlx::SqlitePool;

/// Add a single TotalWarDeckPresetInfo record from a Rust struct.
pub async fn add_total_war_deck_preset_info(
    pool: &SqlitePool,
    data: &TotalWarDeckPresetInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TotalWarDeckPresetInfo (
    Uid,
    Slot,
    PresetName,
    ResourceId,
    ResourceColor,
    DeckInfoIndex
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
    .bind(&data.slot)
    .bind(&data.preset_name)
    .bind(&data.resource_id)
    .bind(&data.resource_color)
    .bind(&data.deck_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_total_war_deck_preset_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<TotalWarDeckPresetInfo>> {
    sqlx::query_as::<_, TotalWarDeckPresetInfo>(
        "SELECT * FROM TotalWarDeckPresetInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Get the preset row for one addressable slot.
pub async fn get_by_uid_and_slot(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
) -> sqlx::Result<Option<TotalWarDeckPresetInfo>> {
    sqlx::query_as::<_, TotalWarDeckPresetInfo>(
        "SELECT * FROM TotalWarDeckPresetInfo WHERE Uid = ? AND Slot = ?",
    )
    .bind(uid)
    .bind(slot)
    .fetch_optional(pool)
    .await
}

/// Highest slot number currently used by this account.
pub async fn max_slot(pool: &SqlitePool, uid: i64) -> sqlx::Result<i32> {
    let row: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(Slot) FROM TotalWarDeckPresetInfo WHERE Uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(row.0.unwrap_or(0))
}

/// Create a new empty preset slot.
pub async fn add_slot(pool: &SqlitePool, uid: i64, slot: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO TotalWarDeckPresetInfo (Uid, Slot) VALUES (?, ?)")
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    Ok(())
}

/// Save/overwrite a preset's name/resource/deck contents for one slot.
pub async fn save_slot(
    pool: &SqlitePool,
    uid: i64,
    slot: i32,
    preset_name: Option<&str>,
    resource_id: Option<i32>,
    resource_color: Option<i32>,
    deck_info_index: Option<&str>,
) -> sqlx::Result<()> {
    if get_by_uid_and_slot(pool, uid, slot).await?.is_some() {
        sqlx::query(
            r#"
UPDATE TotalWarDeckPresetInfo SET
    PresetName = COALESCE(?, PresetName),
    ResourceId = COALESCE(?, ResourceId),
    ResourceColor = COALESCE(?, ResourceColor),
    DeckInfoIndex = COALESCE(?, DeckInfoIndex)
WHERE Uid = ? AND Slot = ?
"#,
        )
        .bind(preset_name)
        .bind(resource_id)
        .bind(resource_color)
        .bind(deck_info_index)
        .bind(uid)
        .bind(slot)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
INSERT INTO TotalWarDeckPresetInfo (Uid, Slot, PresetName, ResourceId, ResourceColor, DeckInfoIndex)
VALUES (?, ?, ?, ?, ?, ?)
"#,
        )
        .bind(uid)
        .bind(slot)
        .bind(preset_name)
        .bind(resource_id)
        .bind(resource_color)
        .bind(deck_info_index)
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
        "DELETE FROM TotalWarDeckPresetInfo WHERE Uid = ? AND Slot IN ({})",
        placeholders
    );
    let mut q = sqlx::query(&query).bind(uid);
    for s in slots {
        q = q.bind(s);
    }
    q.execute(pool).await?;
    Ok(())
}

/// Delete all TotalWarDeckPresetInfo rows for a UID.
pub async fn delete_total_war_deck_preset_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TotalWarDeckPresetInfo WHERE Uid = ?")
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
) -> sqlx::Result<TotalWarDeckPresetInfo> {
    sqlx::query_as::<_, TotalWarDeckPresetInfo>(
        "SELECT * FROM TotalWarDeckPresetInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<TotalWarDeckPresetInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM TotalWarDeckPresetInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TotalWarDeckPresetInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &TotalWarDeckPresetInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO TotalWarDeckPresetInfo (
    Uid,
    Slot,
    PresetName,
    ResourceId,
    ResourceColor,
    DeckInfoIndex
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
    .bind(&data.slot)
    .bind(&data.preset_name)
    .bind(&data.resource_id)
    .bind(&data.resource_color)
    .bind(&data.deck_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
