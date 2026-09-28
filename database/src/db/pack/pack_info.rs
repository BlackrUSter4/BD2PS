use crate::models::game::pack::pack_info::PackInfo;
use sqlx::SqlitePool;

/// Add a single PackInfo record from a Rust struct.
pub async fn add_pack_info(pool: &SqlitePool, data: &PackInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PackInfo (
    Uid,
    Id,
    ClearQuestCount,
    IsPackComplete,
    QuestLevel,
    QuestOpt,
    SubQuestCount,
    ActiveTime,
    IsBuy
) VALUES (
    ?,
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
    .bind(&data.clear_quest_count)
    .bind(&data.is_pack_complete)
    .bind(&data.quest_level)
    .bind(&data.quest_opt)
    .bind(&data.sub_quest_count)
    .bind(&data.active_time)
    .bind(&data.is_buy)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pack_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<PackInfo>> {
    sqlx::query_as::<_, PackInfo>("SELECT * FROM PackInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for one pack id, if saved.
pub async fn get_by_uid_and_id(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
) -> sqlx::Result<Option<PackInfo>> {
    sqlx::query_as::<_, PackInfo>("SELECT * FROM PackInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Get this account's row for a pack id, creating a fresh (not-yet-bought, level 1) one
/// if it's never been touched before.
pub async fn get_or_create(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<PackInfo> {
    if let Some(row) = get_by_uid_and_id(pool, uid, id).await? {
        return Ok(row);
    }
    let fresh = PackInfo {
        index: 0,
        uid,
        id: Some(id),
        clear_quest_count: Some(0),
        is_pack_complete: Some(false),
        quest_level: Some(1),
        quest_opt: Some(0),
        sub_quest_count: Some(0),
        active_time: None,
        is_buy: Some(false),
    };
    add_pack_info(pool, &fresh).await?;
    Ok(get_by_uid_and_id(pool, uid, id)
        .await?
        .expect("row was just inserted"))
}

/// Full-row update, matched by (Uid, Id).
pub async fn update(pool: &SqlitePool, data: &PackInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
UPDATE PackInfo SET
    ClearQuestCount = ?,
    IsPackComplete = ?,
    QuestLevel = ?,
    QuestOpt = ?,
    SubQuestCount = ?,
    ActiveTime = ?,
    IsBuy = ?
WHERE Uid = ? AND Id = ?
"#,
    )
    .bind(&data.clear_quest_count)
    .bind(&data.is_pack_complete)
    .bind(&data.quest_level)
    .bind(&data.quest_opt)
    .bind(&data.sub_quest_count)
    .bind(&data.active_time)
    .bind(&data.is_buy)
    .bind(&data.uid)
    .bind(&data.id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete all PackInfo rows for a UID.
pub async fn delete_pack_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PackInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<PackInfo> {
    sqlx::query_as::<_, PackInfo>("SELECT * FROM PackInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<PackInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PackInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PackInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PackInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PackInfo (
    Uid,
    Id,
    ClearQuestCount,
    IsPackComplete,
    QuestLevel,
    QuestOpt,
    SubQuestCount,
    ActiveTime,
    IsBuy
) VALUES (
    ?,
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
    .bind(&data.clear_quest_count)
    .bind(&data.is_pack_complete)
    .bind(&data.quest_level)
    .bind(&data.quest_opt)
    .bind(&data.sub_quest_count)
    .bind(&data.active_time)
    .bind(&data.is_buy)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
