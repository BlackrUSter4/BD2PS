use crate::models::game::costume::costume_potential_connect_info::CostumePotentialConnectInfo;
use sqlx::SqlitePool;

/// Add a single CostumePotentialConnectInfo record from a Rust struct.
pub async fn add_costume_potential_connect_info(
    pool: &SqlitePool,
    data: &CostumePotentialConnectInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CostumePotentialConnectInfo (
    Uid,
    InvenIndex,
    CostumeId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.costume_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_costume_potential_connect_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CostumePotentialConnectInfo>> {
    sqlx::query_as::<_, CostumePotentialConnectInfo>(
        "SELECT * FROM CostumePotentialConnectInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all CostumePotentialConnectInfo rows for a UID.
pub async fn delete_costume_potential_connect_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CostumePotentialConnectInfo WHERE Uid = ?")
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
) -> sqlx::Result<CostumePotentialConnectInfo> {
    sqlx::query_as::<_, CostumePotentialConnectInfo>(
        "SELECT * FROM CostumePotentialConnectInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CostumePotentialConnectInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CostumePotentialConnectInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CostumePotentialConnectInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CostumePotentialConnectInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CostumePotentialConnectInfo (
    Uid,
    InvenIndex,
    CostumeId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.costume_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
