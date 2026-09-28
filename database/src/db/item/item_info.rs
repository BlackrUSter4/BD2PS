use crate::models::game::item::item_info::ItemInfo;
use sqlx::SqlitePool;

/// Add a single ItemInfo record from a Rust struct.
pub async fn add_item_info(pool: &SqlitePool, data: &ItemInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ItemInfo (
    Uid,
    InvenIndex,
    Id,
    Type,
    Count,
    KeepFlag,
    TimeValue,
    PictorialbookInfoIndex,
    ExpiryTime,
    SortId,
    UseCount
) VALUES (
    ?,
    ?,
    ?,
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
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.r#type)
    .bind(&data.count)
    .bind(&data.keep_flag)
    .bind(&data.time_value)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.expiry_time)
    .bind(&data.sort_id)
    .bind(&data.use_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ItemInfo>> {
    sqlx::query_as::<_, ItemInfo>("SELECT * FROM ItemInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ItemInfo rows for a UID.
pub async fn delete_item_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ItemInfo> {
    sqlx::query_as::<_, ItemInfo>("SELECT * FROM ItemInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ItemInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ItemInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ItemInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Find a stackable item by (Uid, Id), if the user already has one.
pub async fn find_by_item_id(
    pool: &SqlitePool,
    uid: i64,
    item_id: i32,
) -> sqlx::Result<Option<ItemInfo>> {
    sqlx::query_as::<_, ItemInfo>("SELECT * FROM ItemInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(item_id)
        .fetch_optional(pool)
        .await
}

/// Add `count` of an item, stacking onto an existing row of the same Id if present.
pub async fn grant(pool: &SqlitePool, uid: i64, item_id: i32, item_type: i32, count: i32) -> sqlx::Result<()> {
    if let Some(existing) = find_by_item_id(pool, uid, item_id).await? {
        sqlx::query("UPDATE ItemInfo SET Count = Count + ? WHERE \"Index\" = ?")
            .bind(count)
            .bind(existing.index)
            .execute(pool)
            .await?;
    } else {
        insert(
            pool,
            &ItemInfo {
                index: 0,
                uid,
                inven_index: None,
                id: Some(item_id),
                r#type: Some(item_type),
                count: Some(count),
                keep_flag: None,
                time_value: None,
                pictorialbook_info_index: None,
                expiry_time: None,
                sort_id: None,
                use_count: None,
                is_storage: 0,
            },
        )
        .await?;
    }
    Ok(())
}

/// Consume `count` of an item by Id, decrementing (and deleting the row if it hits zero).
/// Returns Err via Ok(false) if the user doesn't have enough.
pub async fn consume(pool: &SqlitePool, uid: i64, item_id: i32, count: i32) -> sqlx::Result<bool> {
    let Some(existing) = find_by_item_id(pool, uid, item_id).await? else {
        return Ok(false);
    };
    let have = existing.count.unwrap_or(0);
    if have < count {
        return Ok(false);
    }
    if have == count {
        sqlx::query("DELETE FROM ItemInfo WHERE \"Index\" = ?")
            .bind(existing.index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE ItemInfo SET Count = Count - ? WHERE \"Index\" = ?")
            .bind(count)
            .bind(existing.index)
            .execute(pool)
            .await?;
    }
    Ok(true)
}

/// Insert and return the rowid (Index)
/// Delete a single item row by its logical InvenIndex (as opposed to the
/// autoincrement rowid `Index` used elsewhere in this module) — this is the
/// identifier `BattleEndRequest.end_inven_index` etc. actually refer to.
pub async fn delete_by_inven_index(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> sqlx::Result<Option<ItemInfo>> {
    sqlx::query_as::<_, ItemInfo>("SELECT * FROM ItemInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_optional(pool)
        .await
}

/// Real per-account items currently moved into storage.
pub async fn get_storage_items(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ItemInfo>> {
    sqlx::query_as::<_, ItemInfo>("SELECT * FROM ItemInfo WHERE Uid = ? AND IsStorage = 1")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn set_storage_flag(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    is_storage: bool,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE ItemInfo SET IsStorage = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(is_storage as i32)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Reduce a stack by `count`, deleting the row outright once it hits zero. Returns false if
/// the account doesn't own that stack or doesn't have enough.
pub async fn reduce_by_inven_index(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    count: i32,
) -> sqlx::Result<bool> {
    let Some(existing) = get_by_inven_index(pool, uid, inven_index).await? else {
        return Ok(false);
    };
    let have = existing.count.unwrap_or(0);
    if have < count {
        return Ok(false);
    }
    if have == count {
        sqlx::query("DELETE FROM ItemInfo WHERE \"Index\" = ?")
            .bind(existing.index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE ItemInfo SET Count = Count - ? WHERE \"Index\" = ?")
            .bind(count)
            .bind(existing.index)
            .execute(pool)
            .await?;
    }
    Ok(true)
}

pub async fn set_keep_flag(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    keep_flag: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE ItemInfo SET KeepFlag = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(keep_flag)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_sort_id(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
    sort_id: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE ItemInfo SET SortId = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(sort_id)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert(pool: &SqlitePool, data: &ItemInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ItemInfo (
    Uid,
    InvenIndex,
    Id,
    Type,
    Count,
    KeepFlag,
    TimeValue,
    PictorialbookInfoIndex,
    ExpiryTime,
    SortId,
    UseCount
) VALUES (
    ?,
    ?,
    ?,
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
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.r#type)
    .bind(&data.count)
    .bind(&data.keep_flag)
    .bind(&data.time_value)
    .bind(&data.pictorialbook_info_index)
    .bind(&data.expiry_time)
    .bind(&data.sort_id)
    .bind(&data.use_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
