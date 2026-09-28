use crate::models::game::mini::mini_game_rhythm_play_info::MiniGameRhythmPlayInfo;
use sqlx::SqlitePool;

/// Add a single MiniGameRhythmPlayInfo record from a Rust struct.
pub async fn add_mini_game_rhythm_play_info(
    pool: &SqlitePool,
    data: &MiniGameRhythmPlayInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MiniGameRhythmPlayInfo (
    Uid,
    Id,
    ModeType,
    BestRecordValue,
    BestGradeType,
    BestComboType
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
    .bind(&data.id)
    .bind(&data.mode_type)
    .bind(&data.best_record_value)
    .bind(&data.best_grade_type)
    .bind(&data.best_combo_type)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mini_game_rhythm_play_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MiniGameRhythmPlayInfo>> {
    sqlx::query_as::<_, MiniGameRhythmPlayInfo>(
        "SELECT * FROM MiniGameRhythmPlayInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MiniGameRhythmPlayInfo rows for a UID.
pub async fn get_by_id(pool: &SqlitePool, uid: i64, id: i32) -> sqlx::Result<Option<MiniGameRhythmPlayInfo>> {
    sqlx::query_as::<_, MiniGameRhythmPlayInfo>("SELECT * FROM MiniGameRhythmPlayInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_best(pool: &SqlitePool, uid: i64, id: i32, mode_type: i32, record_value: i32, grade_type: i32, combo_type: i32) -> sqlx::Result<MiniGameRhythmPlayInfo> {
    if let Some(row) = get_by_id(pool, uid, id).await? {
        if record_value > row.best_record_value.unwrap_or(0) {
            sqlx::query("UPDATE MiniGameRhythmPlayInfo SET BestRecordValue = ?, BestGradeType = ?, BestComboType = ? WHERE \"Index\" = ?")
                .bind(record_value)
                .bind(grade_type)
                .bind(combo_type)
                .bind(row.index)
                .execute(pool)
                .await?;
        }
    } else {
        add_mini_game_rhythm_play_info(pool, &MiniGameRhythmPlayInfo { index: 0, uid, id: Some(id), mode_type: Some(mode_type), best_record_value: Some(record_value), best_grade_type: Some(grade_type), best_combo_type: Some(combo_type) }).await?;
    }
    Ok(get_by_id(pool, uid, id).await?.unwrap())
}

pub async fn delete_mini_game_rhythm_play_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MiniGameRhythmPlayInfo WHERE Uid = ?")
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
) -> sqlx::Result<MiniGameRhythmPlayInfo> {
    sqlx::query_as::<_, MiniGameRhythmPlayInfo>(
        "SELECT * FROM MiniGameRhythmPlayInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MiniGameRhythmPlayInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MiniGameRhythmPlayInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MiniGameRhythmPlayInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MiniGameRhythmPlayInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MiniGameRhythmPlayInfo (
    Uid,
    Id,
    ModeType,
    BestRecordValue,
    BestGradeType,
    BestComboType
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
    .bind(&data.id)
    .bind(&data.mode_type)
    .bind(&data.best_record_value)
    .bind(&data.best_grade_type)
    .bind(&data.best_combo_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
