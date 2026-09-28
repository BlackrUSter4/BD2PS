use crate::models::game::defense::defense_monster_info::DefenseMonsterInfo;
use sqlx::SqlitePool;

/// Add a single DefenseMonsterInfo record from a Rust struct.
pub async fn add_defense_monster_info(
    pool: &SqlitePool,
    data: &DefenseMonsterInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DefenseMonsterInfo (
    Uid,
    MonsterId,
    MonsterIndex,
    SpawnTicks,
    DespwanTicks,
    Health
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
    .bind(&data.monster_id)
    .bind(&data.monster_index)
    .bind(&data.spawn_ticks)
    .bind(&data.despwan_ticks)
    .bind(&data.health)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_defense_monster_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DefenseMonsterInfo>> {
    sqlx::query_as::<_, DefenseMonsterInfo>("SELECT * FROM DefenseMonsterInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DefenseMonsterInfo rows for a UID.
pub async fn delete_defense_monster_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DefenseMonsterInfo WHERE Uid = ?")
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
) -> sqlx::Result<DefenseMonsterInfo> {
    sqlx::query_as::<_, DefenseMonsterInfo>(
        "SELECT * FROM DefenseMonsterInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DefenseMonsterInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DefenseMonsterInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DefenseMonsterInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DefenseMonsterInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DefenseMonsterInfo (
    Uid,
    MonsterId,
    MonsterIndex,
    SpawnTicks,
    DespwanTicks,
    Health
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
    .bind(&data.monster_id)
    .bind(&data.monster_index)
    .bind(&data.spawn_ticks)
    .bind(&data.despwan_ticks)
    .bind(&data.health)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
