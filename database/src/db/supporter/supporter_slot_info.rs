use crate::models::game::supporter::supporter_slot_info::SupporterSlotInfo;
use sqlx::SqlitePool;

/// Add a single SupporterSlotInfo record from a Rust struct.
pub async fn add_supporter_slot_info(
    pool: &SqlitePool,
    data: &SupporterSlotInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SupporterSlotInfo (
    Uid,
    OwnerIndex,
    SlotIndex,
    CostumeId,
    Power,
    BattleUseCount,
    SupporterCharInfo,
    Date
) VALUES (
    ?,
    ?,
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
    .bind(&data.owner_index)
    .bind(&data.slot_index)
    .bind(&data.costume_id)
    .bind(&data.power)
    .bind(&data.battle_use_count)
    .bind(&data.supporter_char_info)
    .bind(&data.date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_supporter_slot_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<SupporterSlotInfo>> {
    sqlx::query_as::<_, SupporterSlotInfo>("SELECT * FROM SupporterSlotInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get one account's registered slot (Uid doubles as the proto's owner_index — this
/// account's own slots are always stored under its own Uid, so cross-account lookups just
/// query by the target's Uid directly).
pub async fn get_by_uid_and_slot(
    pool: &SqlitePool,
    uid: i64,
    slot_index: i32,
) -> sqlx::Result<Option<SupporterSlotInfo>> {
    sqlx::query_as::<_, SupporterSlotInfo>(
        "SELECT * FROM SupporterSlotInfo WHERE Uid = ? AND SlotIndex = ?",
    )
    .bind(uid)
    .bind(slot_index)
    .fetch_optional(pool)
    .await
}

/// Register/overwrite a slot.
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    slot_index: i32,
    costume_id: Option<i32>,
    power: Option<i32>,
    supporter_char_info: Option<&str>,
) -> sqlx::Result<()> {
    if get_by_uid_and_slot(pool, uid, slot_index).await?.is_some() {
        sqlx::query(
            "UPDATE SupporterSlotInfo SET CostumeId = ?, Power = ?, SupporterCharInfo = ?, Date = ? WHERE Uid = ? AND SlotIndex = ?",
        )
        .bind(costume_id)
        .bind(power)
        .bind(supporter_char_info)
        .bind(chrono::Utc::now().timestamp_millis())
        .bind(uid)
        .bind(slot_index)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO SupporterSlotInfo (Uid, OwnerIndex, SlotIndex, CostumeId, Power, BattleUseCount, SupporterCharInfo, Date) VALUES (?, ?, ?, ?, ?, 0, ?, ?)",
        )
        .bind(uid)
        .bind(uid)
        .bind(slot_index)
        .bind(costume_id)
        .bind(power)
        .bind(supporter_char_info)
        .bind(chrono::Utc::now().timestamp_millis())
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Bump a slot's BattleUseCount by 1 (when borrowed).
pub async fn increment_use_count(pool: &SqlitePool, uid: i64, slot_index: i32) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE SupporterSlotInfo SET BattleUseCount = COALESCE(BattleUseCount, 0) + 1 WHERE Uid = ? AND SlotIndex = ?",
    )
    .bind(uid)
    .bind(slot_index)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete one slot.
pub async fn delete_slot(pool: &SqlitePool, uid: i64, slot_index: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SupporterSlotInfo WHERE Uid = ? AND SlotIndex = ?")
        .bind(uid)
        .bind(slot_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all SupporterSlotInfo rows for a UID.
pub async fn delete_supporter_slot_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SupporterSlotInfo WHERE Uid = ?")
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
) -> sqlx::Result<SupporterSlotInfo> {
    sqlx::query_as::<_, SupporterSlotInfo>(
        "SELECT * FROM SupporterSlotInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<SupporterSlotInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SupporterSlotInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SupporterSlotInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SupporterSlotInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SupporterSlotInfo (
    Uid,
    OwnerIndex,
    SlotIndex,
    CostumeId,
    Power,
    BattleUseCount,
    SupporterCharInfo,
    Date
) VALUES (
    ?,
    ?,
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
    .bind(&data.owner_index)
    .bind(&data.slot_index)
    .bind(&data.costume_id)
    .bind(&data.power)
    .bind(&data.battle_use_count)
    .bind(&data.supporter_char_info)
    .bind(&data.date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
