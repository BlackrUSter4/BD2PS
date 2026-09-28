use crate::models::game::equip::equip_info::EquipInfo;
use sqlx::SqlitePool;

/// Insert and return the rowid (Index). InvenIndex is left NULL — callers that need a stable
/// client-facing id should follow up with `set_inven_index_to_own_index`.
pub async fn insert(pool: &SqlitePool, data: &EquipInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EquipInfo (
    Uid,
    InvenIndex,
    UseChar,
    KeepFlag,
    LockFlag,
    BaseInfoIndex
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
    .bind(&data.inven_index)
    .bind(&data.use_char)
    .bind(&data.keep_flag)
    .bind(&data.lock_flag)
    .bind(&data.base_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Stamps InvenIndex = Index for a freshly-inserted row, so the client has a stable id to
/// reference this equip by in later requests (upgrade/lock/break/etc. all key off inven_index).
pub async fn set_inven_index_to_own_index(pool: &SqlitePool, index: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipInfo SET InvenIndex = \"Index\" WHERE \"Index\" = ?")
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_equip_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EquipInfo>> {
    sqlx::query_as::<_, EquipInfo>("SELECT * FROM EquipInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get a single record by InvenIndex (the client-facing id).
pub async fn get_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<EquipInfo>> {
    sqlx::query_as::<_, EquipInfo>("SELECT * FROM EquipInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

/// Get multiple records by their InvenIndex values.
pub async fn get_all_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: &[i64],
) -> sqlx::Result<Vec<EquipInfo>> {
    if inven_index.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = inven_index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EquipInfo WHERE Uid = ? AND InvenIndex IN ({})",
        placeholders
    );
    let mut query = sqlx::query_as::<_, EquipInfo>(&query).bind(uid);
    for idx in inven_index {
        query = query.bind(idx);
    }
    query.fetch_all(pool).await
}

pub async fn set_use_char(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    use_char: Option<i64>,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipInfo SET UseChar = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(use_char)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Read-only: every equip currently marked as equipped on `char_index` (does not clear it,
/// unlike `clear_use_char_for_char`). Used to build a real currently-equipped-gear snapshot
/// for a character wherever one is needed (own preset use, borrowed/opponent character detail).
pub async fn get_by_use_char(
    pool: &SqlitePool,
    uid: i64,
    char_index: i64,
) -> sqlx::Result<Vec<EquipInfo>> {
    sqlx::query_as::<_, EquipInfo>("SELECT * FROM EquipInfo WHERE Uid = ? AND UseChar = ?")
        .bind(uid)
        .bind(char_index)
        .fetch_all(pool)
        .await
}

pub async fn clear_use_char_for_char(
    pool: &SqlitePool,
    uid: i64,
    char_index: i64,
) -> sqlx::Result<Vec<i64>> {
    let rows = sqlx::query_as::<_, EquipInfo>(
        "SELECT * FROM EquipInfo WHERE Uid = ? AND UseChar = ?",
    )
    .bind(uid)
    .bind(char_index)
    .fetch_all(pool)
    .await?;
    sqlx::query("UPDATE EquipInfo SET UseChar = NULL WHERE Uid = ? AND UseChar = ?")
        .bind(uid)
        .bind(char_index)
        .execute(pool)
        .await?;
    Ok(rows.iter().filter_map(|r| r.inven_index).collect())
}

pub async fn set_mark(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    mark: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipInfo SET Mark = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(mark)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_lock_flag(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    lock_flag: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE EquipInfo SET LockFlag = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(lock_flag)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<i64>> {
    let row = get_by_inven_index(pool, uid, inven_index).await?;
    sqlx::query("DELETE FROM EquipInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(row.and_then(|r| r.base_info_index))
}
