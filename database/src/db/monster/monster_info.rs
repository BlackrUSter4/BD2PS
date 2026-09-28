use crate::models::game::monster::monster_info::MonsterInfo;
use sqlx::SqlitePool;

/// Add a single MonsterInfo record from a Rust struct.
pub async fn add_monster_info(pool: &SqlitePool, data: &MonsterInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterInfo (
    Uid,
    MonsterId,
    BattleDeck,
    RespawnTime,
    LifeEndTime,
    GroupId,
    ActiveFlag
) VALUES (
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
    .bind(&data.monster_id)
    .bind(&data.battle_deck)
    .bind(&data.respawn_time)
    .bind(&data.life_end_time)
    .bind(&data.group_id)
    .bind(&data.active_flag)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MonsterInfo>> {
    sqlx::query_as::<_, MonsterInfo>("SELECT * FROM MonsterInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterInfo rows for a UID.
pub async fn delete_monster_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MonsterInfo> {
    sqlx::query_as::<_, MonsterInfo>("SELECT * FROM MonsterInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MonsterInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterInfo (
    Uid,
    MonsterId,
    BattleDeck,
    RespawnTime,
    LifeEndTime,
    GroupId,
    ActiveFlag
) VALUES (
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
    .bind(&data.monster_id)
    .bind(&data.battle_deck)
    .bind(&data.respawn_time)
    .bind(&data.life_end_time)
    .bind(&data.group_id)
    .bind(&data.active_flag)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
