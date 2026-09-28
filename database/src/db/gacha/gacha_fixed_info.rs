use crate::models::game::gacha::gacha_fixed_info::GachaFixedInfo;
use sqlx::SqlitePool;

/// Add a single GachaFixedInfo record from a Rust struct.
pub async fn add_gacha_fixed_info(pool: &SqlitePool, data: &GachaFixedInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaFixedInfo (
    Uid,
    FixedId,
    Type,
    Count,
    ApplySortId
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
    .bind(&data.fixed_id)
    .bind(&data.r#type)
    .bind(&data.count)
    .bind(&data.apply_sort_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_fixed_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaFixedInfo>> {
    sqlx::query_as::<_, GachaFixedInfo>("SELECT * FROM GachaFixedInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaFixedInfo rows for a UID.
pub async fn delete_gacha_fixed_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaFixedInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<GachaFixedInfo> {
    sqlx::query_as::<_, GachaFixedInfo>("SELECT * FROM GachaFixedInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<GachaFixedInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaFixedInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaFixedInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaFixedInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaFixedInfo (
    Uid,
    FixedId,
    Type,
    Count,
    ApplySortId
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
    .bind(&data.fixed_id)
    .bind(&data.r#type)
    .bind(&data.count)
    .bind(&data.apply_sort_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
