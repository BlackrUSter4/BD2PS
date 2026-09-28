use crate::models::game::prestige::prestige_skin_info::PrestigeSkinInfo;
use sqlx::SqlitePool;

/// Add a single PrestigeSkinInfo record from a Rust struct.
pub async fn add_prestige_skin_info(
    pool: &SqlitePool,
    data: &PrestigeSkinInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PrestigeSkinInfo (
    Uid,
    CostumeId,
    CostumeDesignId,
    IsSet
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_id)
    .bind(&data.costume_design_id)
    .bind(&data.is_set)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_prestige_skin_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PrestigeSkinInfo>> {
    sqlx::query_as::<_, PrestigeSkinInfo>("SELECT * FROM PrestigeSkinInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all PrestigeSkinInfo rows for a UID.
pub async fn upsert(pool: &SqlitePool, uid: i64, costume_id: i32, costume_design_id: Option<i32>, is_set: bool) -> sqlx::Result<()> {
    let existing = sqlx::query_as::<_, PrestigeSkinInfo>("SELECT * FROM PrestigeSkinInfo WHERE Uid = ? AND CostumeId = ?")
        .bind(uid)
        .bind(costume_id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = existing {
        sqlx::query("UPDATE PrestigeSkinInfo SET CostumeDesignId = ?, IsSet = ? WHERE \"Index\" = ?")
            .bind(costume_design_id)
            .bind(is_set)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_prestige_skin_info(pool, &PrestigeSkinInfo { index: 0, uid, costume_id: Some(costume_id), costume_design_id, is_set: Some(is_set) }).await?;
    }
    Ok(())
}

pub async fn delete_prestige_skin_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PrestigeSkinInfo WHERE Uid = ?")
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
) -> sqlx::Result<PrestigeSkinInfo> {
    sqlx::query_as::<_, PrestigeSkinInfo>(
        "SELECT * FROM PrestigeSkinInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PrestigeSkinInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PrestigeSkinInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PrestigeSkinInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PrestigeSkinInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PrestigeSkinInfo (
    Uid,
    CostumeId,
    CostumeDesignId,
    IsSet
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.costume_id)
    .bind(&data.costume_design_id)
    .bind(&data.is_set)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
