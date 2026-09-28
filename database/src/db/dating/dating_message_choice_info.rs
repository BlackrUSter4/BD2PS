use crate::models::game::dating::dating_message_choice_info::DatingMessageChoiceInfo;
use sqlx::SqlitePool;

/// Add a single DatingMessageChoiceInfo record from a Rust struct.
pub async fn add_dating_message_choice_info(
    pool: &SqlitePool,
    data: &DatingMessageChoiceInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DatingMessageChoiceInfo (
    Uid,
    GroupId,
    Id,
    SelectTextId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.select_text_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_dating_message_choice_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DatingMessageChoiceInfo>> {
    sqlx::query_as::<_, DatingMessageChoiceInfo>(
        "SELECT * FROM DatingMessageChoiceInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all DatingMessageChoiceInfo rows for a UID.
pub async fn upsert(pool: &SqlitePool, uid: i64, group_id: i32, id: i32, select_text_id: i32) -> sqlx::Result<()> {
    let existing = sqlx::query_as::<_, DatingMessageChoiceInfo>(
        "SELECT * FROM DatingMessageChoiceInfo WHERE Uid = ? AND GroupId = ? AND Id = ?",
    )
    .bind(uid)
    .bind(group_id)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = existing {
        sqlx::query("UPDATE DatingMessageChoiceInfo SET SelectTextId = ? WHERE \"Index\" = ?")
            .bind(select_text_id)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_dating_message_choice_info(
            pool,
            &DatingMessageChoiceInfo { index: 0, uid, group_id: Some(group_id), id: Some(id), select_text_id: Some(select_text_id) },
        )
        .await?;
    }
    Ok(())
}

pub async fn delete_dating_message_choice_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DatingMessageChoiceInfo WHERE Uid = ?")
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
) -> sqlx::Result<DatingMessageChoiceInfo> {
    sqlx::query_as::<_, DatingMessageChoiceInfo>(
        "SELECT * FROM DatingMessageChoiceInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DatingMessageChoiceInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DatingMessageChoiceInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DatingMessageChoiceInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DatingMessageChoiceInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DatingMessageChoiceInfo (
    Uid,
    GroupId,
    Id,
    SelectTextId
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.select_text_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
