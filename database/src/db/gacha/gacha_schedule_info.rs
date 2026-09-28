use crate::models::game::gacha::gacha_schedule_info::GachaScheduleInfo;
use sqlx::SqlitePool;

/// Add a single GachaScheduleInfo record from a Rust struct.
pub async fn add_gacha_schedule_info(
    pool: &SqlitePool,
    data: &GachaScheduleInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaScheduleInfo (
    Uid,
    GroupId,
    StartTime,
    EndTime,
    IsGachaFreeCountBonus,
    IsGachaCashCountBonus
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
    .bind(&data.group_id)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.is_gacha_free_count_bonus)
    .bind(&data.is_gacha_cash_count_bonus)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_schedule_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaScheduleInfo>> {
    sqlx::query_as::<_, GachaScheduleInfo>("SELECT * FROM GachaScheduleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaScheduleInfo rows for a UID.
pub async fn delete_gacha_schedule_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaScheduleInfo WHERE Uid = ?")
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
) -> sqlx::Result<GachaScheduleInfo> {
    sqlx::query_as::<_, GachaScheduleInfo>(
        "SELECT * FROM GachaScheduleInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GachaScheduleInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaScheduleInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaScheduleInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaScheduleInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaScheduleInfo (
    Uid,
    GroupId,
    StartTime,
    EndTime,
    IsGachaFreeCountBonus,
    IsGachaCashCountBonus
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
    .bind(&data.group_id)
    .bind(&data.start_time)
    .bind(&data.end_time)
    .bind(&data.is_gacha_free_count_bonus)
    .bind(&data.is_gacha_cash_count_bonus)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
