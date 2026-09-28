use crate::models::game::statue::statue_info::StatueInfo;
use sqlx::SqlitePool;

/// Add a single StatueInfo record from a Rust struct.
pub async fn add_statue_info(pool: &SqlitePool, data: &StatueInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO StatueInfo (
    Uid,
    Id,
    Season,
    Rank,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PortraitCostumeDesignId
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
    .bind(&data.id)
    .bind(&data.season)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_statue_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<StatueInfo>> {
    sqlx::query_as::<_, StatueInfo>("SELECT * FROM StatueInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all StatueInfo rows for a UID.
pub async fn delete_statue_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM StatueInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<StatueInfo> {
    sqlx::query_as::<_, StatueInfo>("SELECT * FROM StatueInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<StatueInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM StatueInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, StatueInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &StatueInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO StatueInfo (
    Uid,
    Id,
    Season,
    Rank,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PortraitCostumeDesignId
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
    .bind(&data.id)
    .bind(&data.season)
    .bind(&data.rank)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
