use crate::models::game::talent::talent_npc_info::TalentNpcInfo;
use sqlx::SqlitePool;

/// Add a single TalentNpcInfo record from a Rust struct.
pub async fn add_talent_npc_info(pool: &SqlitePool, data: &TalentNpcInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TalentNpcInfo (
    Uid,
    NpcId,
    GroupId,
    EndTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.npc_id)
    .bind(&data.group_id)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_talent_npc_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<TalentNpcInfo>> {
    sqlx::query_as::<_, TalentNpcInfo>("SELECT * FROM TalentNpcInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all TalentNpcInfo rows for a UID.
pub async fn delete_talent_npc_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TalentNpcInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<TalentNpcInfo> {
    sqlx::query_as::<_, TalentNpcInfo>("SELECT * FROM TalentNpcInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<TalentNpcInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM TalentNpcInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TalentNpcInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &TalentNpcInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO TalentNpcInfo (
    Uid,
    NpcId,
    GroupId,
    EndTime
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.npc_id)
    .bind(&data.group_id)
    .bind(&data.end_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
