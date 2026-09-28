use crate::models::game::price::price_info::PriceInfo;
use sqlx::SqlitePool;

/// Add a single PriceInfo record from a Rust struct.
pub async fn add_price_info(pool: &SqlitePool, data: &PriceInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PriceInfo (
    Uid,
    PriceCurrencyCode,
    Price
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.price_currency_code)
    .bind(&data.price)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_price_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PriceInfo>> {
    sqlx::query_as::<_, PriceInfo>("SELECT * FROM PriceInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PriceInfo rows for a UID.
pub async fn delete_price_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PriceInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<PriceInfo> {
    sqlx::query_as::<_, PriceInfo>("SELECT * FROM PriceInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<PriceInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PriceInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PriceInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PriceInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PriceInfo (
    Uid,
    PriceCurrencyCode,
    Price
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.price_currency_code)
    .bind(&data.price)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
