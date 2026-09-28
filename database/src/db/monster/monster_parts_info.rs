use crate::models::game::monster::monster_parts_info::MonsterPartsInfo;
use sqlx::SqlitePool;

/// Add a single MonsterPartsInfo record from a Rust struct.
pub async fn add_monster_parts_info(
    pool: &SqlitePool,
    data: &MonsterPartsInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterPartsInfo (
    Uid,
    PartsId,
    PartsHealth
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.parts_id)
    .bind(&data.parts_health)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_parts_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterPartsInfo>> {
    sqlx::query_as::<_, MonsterPartsInfo>("SELECT * FROM MonsterPartsInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterPartsInfo rows for a UID.
pub async fn delete_monster_parts_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterPartsInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterPartsInfo> {
    sqlx::query_as::<_, MonsterPartsInfo>(
        "SELECT * FROM MonsterPartsInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterPartsInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterPartsInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterPartsInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterPartsInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterPartsInfo (
    Uid,
    PartsId,
    PartsHealth
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.parts_id)
    .bind(&data.parts_health)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
