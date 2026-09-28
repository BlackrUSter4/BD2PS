use crate::models::game::item::item_auto_exchange_info::ItemAutoExchangeInfo;
use sqlx::SqlitePool;

/// Add a single ItemAutoExchangeInfo record from a Rust struct.
pub async fn add_item_auto_exchange_info(
    pool: &SqlitePool,
    data: &ItemAutoExchangeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ItemAutoExchangeInfo (
    Uid,
    OriginalItemType,
    OriginalItemId,
    OriginalItemCount,
    ExchangeItemType,
    ExchangeItemId,
    ExchangeItemCount,
    SortId
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
    .bind(&data.original_item_type)
    .bind(&data.original_item_id)
    .bind(&data.original_item_count)
    .bind(&data.exchange_item_type)
    .bind(&data.exchange_item_id)
    .bind(&data.exchange_item_count)
    .bind(&data.sort_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_item_auto_exchange_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ItemAutoExchangeInfo>> {
    sqlx::query_as::<_, ItemAutoExchangeInfo>("SELECT * FROM ItemAutoExchangeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ItemAutoExchangeInfo rows for a UID.
pub async fn delete_item_auto_exchange_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ItemAutoExchangeInfo WHERE Uid = ?")
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
) -> sqlx::Result<ItemAutoExchangeInfo> {
    sqlx::query_as::<_, ItemAutoExchangeInfo>(
        "SELECT * FROM ItemAutoExchangeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<ItemAutoExchangeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ItemAutoExchangeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ItemAutoExchangeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ItemAutoExchangeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ItemAutoExchangeInfo (
    Uid,
    OriginalItemType,
    OriginalItemId,
    OriginalItemCount,
    ExchangeItemType,
    ExchangeItemId,
    ExchangeItemCount,
    SortId
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
    .bind(&data.original_item_type)
    .bind(&data.original_item_id)
    .bind(&data.original_item_count)
    .bind(&data.exchange_item_type)
    .bind(&data.exchange_item_id)
    .bind(&data.exchange_item_count)
    .bind(&data.sort_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
