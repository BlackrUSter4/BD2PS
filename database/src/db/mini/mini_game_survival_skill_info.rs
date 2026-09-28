use crate::models::game::mini::mini_game_survival_skill_info::MiniGameSurvivalSkillInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameSurvivalSkillInfo record from a Rust struct.
pub async fn add_mini_game_survival_skill_info(
    pool: &SqlitePool,
    data: &MiniGameSurvivalSkillInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalSkillInfo (
    Uid,
    SkillId,
    Level,
    Dps
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.skill_id)
    .bind(&data.level)
    .bind(&data.dps)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_survival_skill_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameSurvivalSkillInfo>> {
    sqlx::query_as::<_, MiniGameSurvivalSkillInfo>(
        "SELECT * FROM MiniGameSurvivalSkillInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Set (insert or update) the level for a given skill_id.
pub async fn upsert_level(pool: &SqlitePool, uid: i64, skill_id: i32, level: i32, dps: Option<i32>) -> sqlx::Result<()> {
    let existing = get_mini_game_survival_skill_info(pool, uid)
        .await?
        .into_iter()
        .find(|r| r.skill_id == Some(skill_id));

    if let Some(row) = existing {
        sqlx::query("UPDATE MiniGameSurvivalSkillInfo SET Level = ?, Dps = ? WHERE \"Index\" = ?")
            .bind(level)
            .bind(dps)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_mini_game_survival_skill_info(
            pool,
            &MiniGameSurvivalSkillInfo { index: 0, uid, skill_id: Some(skill_id), level: Some(level), dps },
        )
        .await?;
    }
    Ok(())
}

/// Delete all MiniGameSurvivalSkillInfo rows for a UID.
pub async fn delete_mini_game_survival_skill_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameSurvivalSkillInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameSurvivalSkillInfo> {
    sqlx::query_as::<_, MiniGameSurvivalSkillInfo>(
        "SELECT * FROM MiniGameSurvivalSkillInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameSurvivalSkillInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameSurvivalSkillInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameSurvivalSkillInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameSurvivalSkillInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameSurvivalSkillInfo (
    Uid,
    SkillId,
    Level,
    Dps
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.skill_id)
    .bind(&data.level)
    .bind(&data.dps)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
