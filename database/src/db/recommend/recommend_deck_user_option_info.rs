use crate::models::game::recommend::recommend_deck_user_option_info::RecommendDeckUserOptionInfo;
use sqlx::SqlitePool;

/// Add a single RecommendDeckUserOptionInfo record from a Rust struct.
pub async fn add_recommend_deck_user_option_info(
    pool: &SqlitePool,
    data: &RecommendDeckUserOptionInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RecommendDeckUserOptionInfo (
    Uid,
    IsSave,
    IsOnlyCharOwnDisplay
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.is_save)
    .bind(&data.is_only_char_own_display)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_recommend_deck_user_option_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<RecommendDeckUserOptionInfo>> {
    sqlx::query_as::<_, RecommendDeckUserOptionInfo>(
        "SELECT * FROM RecommendDeckUserOptionInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all RecommendDeckUserOptionInfo rows for a UID.
pub async fn upsert(pool: &SqlitePool, uid: i64, is_save: bool, is_only_char_own_display: bool) -> sqlx::Result<()> {
    let existing = get_recommend_deck_user_option_info(pool, uid).await?;
    if let Some(row) = existing.into_iter().next() {
        sqlx::query("UPDATE RecommendDeckUserOptionInfo SET IsSave = ?, IsOnlyCharOwnDisplay = ? WHERE \"Index\" = ?")
            .bind(is_save)
            .bind(is_only_char_own_display)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_recommend_deck_user_option_info(
            pool,
            &RecommendDeckUserOptionInfo { index: 0, uid, is_save: Some(is_save), is_only_char_own_display: Some(is_only_char_own_display) },
        )
        .await?;
    }
    Ok(())
}

pub async fn delete_recommend_deck_user_option_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RecommendDeckUserOptionInfo WHERE Uid = ?")
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
) -> sqlx::Result<RecommendDeckUserOptionInfo> {
    sqlx::query_as::<_, RecommendDeckUserOptionInfo>(
        "SELECT * FROM RecommendDeckUserOptionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<RecommendDeckUserOptionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RecommendDeckUserOptionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RecommendDeckUserOptionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RecommendDeckUserOptionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RecommendDeckUserOptionInfo (
    Uid,
    IsSave,
    IsOnlyCharOwnDisplay
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.is_save)
    .bind(&data.is_only_char_own_display)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
