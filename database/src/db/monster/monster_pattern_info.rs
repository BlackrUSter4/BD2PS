use crate::models::game::monster::monster_pattern_info::MonsterPatternInfo;
use sqlx::SqlitePool;

/// Add a single MonsterPatternInfo record from a Rust struct.
pub async fn add_monster_pattern_info(
    pool: &SqlitePool,
    data: &MonsterPatternInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterPatternInfo (
    Uid,
    TargetOwnerIndex,
    PatternId,
    DistanceToAttack
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.target_owner_index)
    .bind(&data.pattern_id)
    .bind(&data.distance_to_attack)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_pattern_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterPatternInfo>> {
    sqlx::query_as::<_, MonsterPatternInfo>("SELECT * FROM MonsterPatternInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterPatternInfo rows for a UID.
pub async fn delete_monster_pattern_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterPatternInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterPatternInfo> {
    sqlx::query_as::<_, MonsterPatternInfo>(
        "SELECT * FROM MonsterPatternInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterPatternInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterPatternInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterPatternInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterPatternInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterPatternInfo (
    Uid,
    TargetOwnerIndex,
    PatternId,
    DistanceToAttack
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.target_owner_index)
    .bind(&data.pattern_id)
    .bind(&data.distance_to_attack)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
