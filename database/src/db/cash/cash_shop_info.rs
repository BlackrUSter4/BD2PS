use crate::models::game::cash::cash_shop_info::CashShopInfo;
use sqlx::SqlitePool;

/// Add a single CashShopInfo record from a Rust struct.
pub async fn add_cash_shop_info(pool: &SqlitePool, data: &CashShopInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CashShopInfo (
    Uid,
    GroupId,
    ShopId,
    StartTime,
    EndTime
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
    .bind(&data.group_id)
    .bind(&data.shop_id)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cash_shop_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CashShopInfo>> {
    sqlx::query_as::<_, CashShopInfo>("SELECT * FROM CashShopInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CashShopInfo rows for a UID.
pub async fn delete_cash_shop_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CashShopInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CashShopInfo> {
    sqlx::query_as::<_, CashShopInfo>("SELECT * FROM CashShopInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<CashShopInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CashShopInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CashShopInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CashShopInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CashShopInfo (
    Uid,
    GroupId,
    ShopId,
    StartTime,
    EndTime
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
    .bind(&data.group_id)
    .bind(&data.shop_id)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
