use crate::models::game::id::id_card_info::IdCardInfo;
use sqlx::SqlitePool;

/// Add a single IdCardInfo record from a Rust struct.
pub async fn add_id_card_info(pool: &SqlitePool, data: &IdCardInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO IdCardInfo (
    Uid,
    Background,
    SubBackground,
    BackgroundEffect,
    MyInfo,
    Rotate
) VALUES (
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
    .bind(&data.background_index)
    .bind(&data.sub_background_index)
    .bind(&data.background_effect_index)
    .bind(&data.my_info_index)
    .bind(&data.rotate)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_id_card_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<IdCardInfo>> {
    sqlx::query_as::<_, IdCardInfo>("SELECT * FROM IdCardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the account's "current" IdCardInfo row. Judgment call: this table also holds one
/// independent row per saved preset (see `store_card_fresh` in the id module) with no
/// column distinguishing "current" from "preset snapshot" — approximated as the
/// most-recently-inserted row not referenced by any preset, which is correct as long as
/// `IdCardSaveRequest` (which always targets this "current" row) is called after whatever
/// preset saves preceded it, which is the expected client flow.
pub async fn get_current(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<IdCardInfo>> {
    sqlx::query_as::<_, IdCardInfo>(
        r#"
SELECT * FROM IdCardInfo
WHERE Uid = ? AND "Index" NOT IN (
    SELECT IdCardInfoIndex FROM IdCardPresetInfo WHERE Uid = ? AND IdCardInfoIndex IS NOT NULL
)
ORDER BY "Index" DESC LIMIT 1
"#,
    )
    .bind(uid)
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// Full-row update, matched by Uid (there is exactly one current IdCardInfo row per
/// account).
pub async fn update(pool: &SqlitePool, data: &IdCardInfo) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE IdCardInfo SET BackgroundIndex = ?, SubBackgroundIndex = ?, BackgroundEffectIndex = ?, StickersIndex = ?, MyInfoIndex = ?, Rotate = ? WHERE Uid = ?",
    )
    .bind(&data.background_index)
    .bind(&data.sub_background_index)
    .bind(&data.background_effect_index)
    .bind(&data.stickers_index)
    .bind(&data.my_info_index)
    .bind(&data.rotate)
    .bind(&data.uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete all IdCardInfo rows for a UID.
pub async fn delete_id_card_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM IdCardInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<IdCardInfo> {
    sqlx::query_as::<_, IdCardInfo>("SELECT * FROM IdCardInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<IdCardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM IdCardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, IdCardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &IdCardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO IdCardInfo (
    Uid,
    Background,
    SubBackground,
    BackgroundEffect,
    MyInfo,
    Rotate
) VALUES (
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
    .bind(&data.background_index)
    .bind(&data.sub_background_index)
    .bind(&data.background_effect_index)
    .bind(&data.my_info_index)
    .bind(&data.rotate)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
