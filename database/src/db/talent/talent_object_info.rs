use crate::models::game::talent::talent_object_info::TalentObjectInfo;
use sqlx::SqlitePool;

/// Add a single TalentObjectInfo record from a Rust struct.
pub async fn add_talent_object_info(
    pool: &SqlitePool,
    data: &TalentObjectInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TalentObjectInfo (
    Uid,
    TalentType,
    ObjectId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.talent_type)
    .bind(&data.object_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_talent_object_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<TalentObjectInfo>> {
    sqlx::query_as::<_, TalentObjectInfo>("SELECT * FROM TalentObjectInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all TalentObjectInfo rows for a UID.
pub async fn delete_talent_object_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TalentObjectInfo WHERE Uid = ?")
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
) -> sqlx::Result<TalentObjectInfo> {
    sqlx::query_as::<_, TalentObjectInfo>(
        "SELECT * FROM TalentObjectInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<TalentObjectInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM TalentObjectInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TalentObjectInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &TalentObjectInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO TalentObjectInfo (
    Uid,
    TalentType,
    ObjectId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.talent_type)
    .bind(&data.object_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
