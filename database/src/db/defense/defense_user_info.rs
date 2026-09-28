use crate::models::game::defense::defense_user_info::DefenseUserInfo;
use sqlx::SqlitePool;

/// Add a single DefenseUserInfo record from a Rust struct.
pub async fn add_defense_user_info(pool: &SqlitePool, data: &DefenseUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DefenseUserInfo (
    Uid,
    OwnerIndex,
    State,
    EnemyCount,
    Wave,
    TotalEnemyKillCount,
    NetworkState,
    NetworkPing
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.state)
    .bind(&data.enemy_count)
    .bind(&data.wave)
    .bind(&data.total_enemy_kill_count)
    .bind(&data.network_state)
    .bind(&data.network_ping)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_defense_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DefenseUserInfo>> {
    sqlx::query_as::<_, DefenseUserInfo>("SELECT * FROM DefenseUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all DefenseUserInfo rows for a UID.
pub async fn delete_defense_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DefenseUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<DefenseUserInfo> {
    sqlx::query_as::<_, DefenseUserInfo>(
        "SELECT * FROM DefenseUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<DefenseUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM DefenseUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, DefenseUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DefenseUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DefenseUserInfo (
    Uid,
    OwnerIndex,
    State,
    EnemyCount,
    Wave,
    TotalEnemyKillCount,
    NetworkState,
    NetworkPing
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.state)
    .bind(&data.enemy_count)
    .bind(&data.wave)
    .bind(&data.total_enemy_kill_count)
    .bind(&data.network_state)
    .bind(&data.network_ping)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
