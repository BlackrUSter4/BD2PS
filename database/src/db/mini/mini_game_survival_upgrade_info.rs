use crate::models::game::mini::mini_game_survival_upgrade_info::MiniGameSurvivalUpgradeInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSurvivalUpgradeInfo record from a Rust struct.
pub async fn add_mini_game_survival_upgrade_info(
    pool: &SqlitePool,
    data: &MiniGameSurvivalUpgradeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalUpgradeInfo (
    Uid,
    UpgradeId,
    Level
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.upgrade_id)
    .bind(&data.level)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_survival_upgrade_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSurvivalUpgradeInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalUpgradeInfo>(
        "SELECT * FROM MiniGameSurvivalUpgradeInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Set (insert or update) the level for a given upgrade_id, returning the new level.
pub async fn upsert_level(pool: &SqlitePool, uid: i64, upgrade_id: i32, level: i32) -> sqlx::Result<()> {
    let existing = get_mini_game_survival_upgrade_info(pool, uid)
        .await?
        .into_iter()
        .find(|r| r.upgrade_id == Some(upgrade_id));

    if let Some(row) = existing {
        sqlx::query("UPDATE MiniGameSurvivalUpgradeInfo SET Level = ? WHERE \"Index\" = ?")
            .bind(level)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_survival_upgrade_info(
            pool,
            &MiniGameSurvivalUpgradeInfo { index: 0, uid, upgrade_id: Some(upgrade_id), level: Some(level) },
        )
        .await?;
    }
    Ok(())
}

/// Delete all MiniGameSurvivalUpgradeInfo rows for a UID.
pub async fn delete_mini_game_survival_upgrade_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSurvivalUpgradeInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSurvivalUpgradeInfo> {
    sqlx::query_as::<_, MiniGameSurvivalUpgradeInfo>(
        "SELECT * FROM MiniGameSurvivalUpgradeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSurvivalUpgradeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSurvivalUpgradeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSurvivalUpgradeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSurvivalUpgradeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalUpgradeInfo (
    Uid,
    UpgradeId,
    Level
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.upgrade_id)
    .bind(&data.level)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
