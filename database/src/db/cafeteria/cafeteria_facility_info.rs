use crate::models::game::cafeteria::cafeteria_facility_info::CafeteriaFacilityInfo;
use sqlx::SqlitePool;

/// Add a single CafeteriaFacilityInfo record from a Rust struct.
pub async fn add_cafeteria_facility_info(
    pool: &SqlitePool,
    data: &CafeteriaFacilityInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CafeteriaFacilityInfo (
    Uid,
    FacilityId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.facility_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cafeteria_facility_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CafeteriaFacilityInfo>> {
    sqlx::query_as::<_, CafeteriaFacilityInfo>("SELECT * FROM CafeteriaFacilityInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CafeteriaFacilityInfo rows for a UID.
pub async fn delete_cafeteria_facility_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CafeteriaFacilityInfo WHERE Uid = ?")
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
) -> sqlx::Result<CafeteriaFacilityInfo> {
    sqlx::query_as::<_, CafeteriaFacilityInfo>(
        "SELECT * FROM CafeteriaFacilityInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CafeteriaFacilityInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CafeteriaFacilityInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CafeteriaFacilityInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CafeteriaFacilityInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CafeteriaFacilityInfo (
    Uid,
    FacilityId
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.facility_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
