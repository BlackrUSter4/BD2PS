use crate::models::game::pack::pack_sub_quest_clear_info::PackSubQuestClearInfo;
use sqlx::SqlitePool;

/// Add a single PackSubQuestClearInfo record from a Rust struct.
pub async fn add_pack_sub_quest_clear_info(
    pool: &SqlitePool,
    data: &PackSubQuestClearInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackSubQuestClearInfo (
    Uid,
    PackId,
    SubQuestClearCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.sub_quest_clear_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_sub_quest_clear_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PackSubQuestClearInfo>> {
    sqlx::query_as::<_, PackSubQuestClearInfo>("SELECT * FROM PackSubQuestClearInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for one pack, if saved.
pub async fn get_by_uid_and_pack(
    pool: &SqlitePool,
    uid: i64,
    pack_id: i32,
) -> sqlx::Result<Option<PackSubQuestClearInfo>> {
    sqlx::query_as::<_, PackSubQuestClearInfo>(
        "SELECT * FROM PackSubQuestClearInfo WHERE Uid = ? AND PackId = ?",
    )
    .bind(uid)
    .bind(pack_id)
    .fetch_optional(pool)
    .await
}

/// Delete all PackSubQuestClearInfo rows for a UID.
pub async fn delete_pack_sub_quest_clear_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackSubQuestClearInfo WHERE Uid = ?")
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
) -> sqlx::Result<PackSubQuestClearInfo> {
    sqlx::query_as::<_, PackSubQuestClearInfo>(
        "SELECT * FROM PackSubQuestClearInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<PackSubQuestClearInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackSubQuestClearInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackSubQuestClearInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackSubQuestClearInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackSubQuestClearInfo (
    Uid,
    PackId,
    SubQuestClearCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.sub_quest_clear_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
