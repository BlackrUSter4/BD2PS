use crate::models::game::event::event_exchange_info::EventExchangeInfo;
use sqlx::SqlitePool;

/// Add a single EventExchangeInfo record from a Rust struct.
pub async fn add_event_exchange_info(
    pool: &SqlitePool,
    data: &EventExchangeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EventExchangeInfo (
    Uid,
    EventUid,
    GroupId,
    Page,
    KeyCount
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
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.page)
    .bind(&data.key_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_event_exchange_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EventExchangeInfo>> {
    sqlx::query_as::<_, EventExchangeInfo>("SELECT * FROM EventExchangeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for one (event_uid, group_id) pair, if any progress has been made.
pub async fn get_by_uid_event_group(
    pool: &SqlitePool,
    uid: i64,
    event_uid: i32,
    group_id: i32,
) -> sqlx::Result<Option<EventExchangeInfo>> {
    sqlx::query_as::<_, EventExchangeInfo>(
        "SELECT * FROM EventExchangeInfo WHERE Uid = ? AND EventUid = ? AND GroupId = ?",
    )
    .bind(uid)
    .bind(event_uid)
    .bind(group_id)
    .fetch_optional(pool)
    .await
}

/// Get or create the account's exchange-progress row for one (event_uid, group_id).
pub async fn get_or_create(
    pool: &SqlitePool,
    uid: i64,
    event_uid: i32,
    group_id: i32,
) -> sqlx::Result<EventExchangeInfo> {
    if let Some(row) = get_by_uid_event_group(pool, uid, event_uid, group_id).await? {
        return Ok(row);
    }
    add_event_exchange_info(
        pool,
        &EventExchangeInfo { index: 0, uid, event_uid: Some(event_uid), group_id: Some(group_id), page: Some(1), key_count: Some(0) },
    )
    .await?;
    Ok(get_by_uid_event_group(pool, uid, event_uid, group_id).await?.expect("row was just inserted"))
}

/// Bump the page counter for one (event_uid, group_id).
pub async fn open_next_page(pool: &SqlitePool, uid: i64, event_uid: i32, group_id: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid, event_uid, group_id).await?;
    sqlx::query("UPDATE EventExchangeInfo SET Page = COALESCE(Page, 1) + 1 WHERE Uid = ? AND EventUid = ? AND GroupId = ?")
        .bind(uid)
        .bind(event_uid)
        .bind(group_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Add to the key_count for one (event_uid, group_id).
pub async fn add_key_count(pool: &SqlitePool, uid: i64, event_uid: i32, group_id: i32, delta: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid, event_uid, group_id).await?;
    sqlx::query("UPDATE EventExchangeInfo SET KeyCount = COALESCE(KeyCount, 0) + ? WHERE Uid = ? AND EventUid = ? AND GroupId = ?")
        .bind(delta)
        .bind(uid)
        .bind(event_uid)
        .bind(group_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all EventExchangeInfo rows for a UID.
pub async fn delete_event_exchange_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EventExchangeInfo WHERE Uid = ?")
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
) -> sqlx::Result<EventExchangeInfo> {
    sqlx::query_as::<_, EventExchangeInfo>(
        "SELECT * FROM EventExchangeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EventExchangeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EventExchangeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EventExchangeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EventExchangeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EventExchangeInfo (
    Uid,
    EventUid,
    GroupId,
    Page,
    KeyCount
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
    .bind(&data.event_uid)
    .bind(&data.group_id)
    .bind(&data.page)
    .bind(&data.key_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
