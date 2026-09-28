use crate::models::game::cash::cash_product_info::CashProductInfo;
use sqlx::SqlitePool;

/// Add a single CashProductInfo record from a Rust struct.
pub async fn add_cash_product_info(pool: &SqlitePool, data: &CashProductInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CashProductInfo (
    Uid,
    GroupId,
    Id,
    SaleGroup,
    StartTime,
    EndTime,
    EndDelayMinutes,
    EventIndex
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.sale_group)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.end_delay_minutes)
    .bind(&data.event_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cash_product_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CashProductInfo>> {
    sqlx::query_as::<_, CashProductInfo>("SELECT * FROM CashProductInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CashProductInfo rows for a UID.
pub async fn delete_cash_product_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CashProductInfo WHERE Uid = ?")
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
) -> sqlx::Result<CashProductInfo> {
    sqlx::query_as::<_, CashProductInfo>(
        "SELECT * FROM CashProductInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CashProductInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CashProductInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CashProductInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CashProductInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CashProductInfo (
    Uid,
    GroupId,
    Id,
    SaleGroup,
    StartTime,
    EndTime,
    EndDelayMinutes,
    EventIndex
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
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.sale_group)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.end_delay_minutes)
    .bind(&data.event_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
