use crate::models::game::gacha::gacha_user_info::GachaUserInfo;
use sqlx::SqlitePool;

/// Add a single GachaUserInfo record from a Rust struct.
pub async fn add_gacha_user_info(pool: &SqlitePool, data: &GachaUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaUserInfo (
    Uid,
    GroupId,
    Point,
    TotalBuyCount,
    OneFreePickCount,
    OneCashPickCount,
    TenFreePickCount,
    TenCashPickCount,
    ExchangeItemCount,
    ExchangeMileageCount
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.point)
    .bind(&data.total_buy_count)
    .bind(&data.one_free_pick_count)
    .bind(&data.one_cash_pick_count)
    .bind(&data.ten_free_pick_count)
    .bind(&data.ten_cash_pick_count)
    .bind(&data.exchange_item_count)
    .bind(&data.exchange_mileage_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GachaUserInfo>> {
    sqlx::query_as::<_, GachaUserInfo>("SELECT * FROM GachaUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaUserInfo rows for a UID.
pub async fn delete_gacha_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<GachaUserInfo> {
    sqlx::query_as::<_, GachaUserInfo>("SELECT * FROM GachaUserInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<GachaUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaUserInfo (
    Uid,
    GroupId,
    Point,
    TotalBuyCount,
    OneFreePickCount,
    OneCashPickCount,
    TenFreePickCount,
    TenCashPickCount,
    ExchangeItemCount,
    ExchangeMileageCount
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.point)
    .bind(&data.total_buy_count)
    .bind(&data.one_free_pick_count)
    .bind(&data.one_cash_pick_count)
    .bind(&data.ten_free_pick_count)
    .bind(&data.ten_cash_pick_count)
    .bind(&data.exchange_item_count)
    .bind(&data.exchange_mileage_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
