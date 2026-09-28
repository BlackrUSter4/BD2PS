use crate::models::game::id::id_card_preset_info::IdCardPresetInfo;
use sqlx::SqlitePool;

/// Add a single IdCardPresetInfo record from a Rust struct.
pub async fn add_id_card_preset_info(
    pool: &SqlitePool,
    data: &IdCardPresetInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO IdCardPresetInfo (
    Uid,
    Id,
    IdCardInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.id_card_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_id_card_preset_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<IdCardPresetInfo>> {
    sqlx::query_as::<_, IdCardPresetInfo>("SELECT * FROM IdCardPresetInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the preset row for one id, if saved.
pub async fn get_by_uid_and_id(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
) -> sqlx::Result<Option<IdCardPresetInfo>> {
    sqlx::query_as::<_, IdCardPresetInfo>("SELECT * FROM IdCardPresetInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Save/overwrite a preset's stored id-card snapshot index.
pub async fn upsert(pool: &SqlitePool, uid: i64, id: i32, id_card_info_index: i64) -> sqlx::Result<()> {
    if get_by_uid_and_id(pool, uid, id).await?.is_some() {
        sqlx::query("UPDATE IdCardPresetInfo SET IdCardInfoIndex = ? WHERE Uid = ? AND Id = ?")
            .bind(id_card_info_index)
            .bind(uid)
            .bind(id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO IdCardPresetInfo (Uid, Id, IdCardInfoIndex) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(id)
            .bind(id_card_info_index)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Delete one preset by id.
pub async fn delete_by_id(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IdCardPresetInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all IdCardPresetInfo rows for a UID.
pub async fn delete_id_card_preset_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IdCardPresetInfo WHERE Uid = ?")
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
) -> sqlx::Result<IdCardPresetInfo> {
    sqlx::query_as::<_, IdCardPresetInfo>(
        "SELECT * FROM IdCardPresetInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<IdCardPresetInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM IdCardPresetInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, IdCardPresetInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &IdCardPresetInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO IdCardPresetInfo (
    Uid,
    Id,
    IdCardInfo
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.id_card_info_index)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
