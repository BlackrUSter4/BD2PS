use crate::models::game::gacha::gacha_step_up_user_info::GachaStepUpUserInfo;
use sqlx::SqlitePool;

/// Add a single GachaStepUpUserInfo record from a Rust struct.
pub async fn add_gacha_step_up_user_info(
    pool: &SqlitePool,
    data: &GachaStepUpUserInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GachaStepUpUserInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_gacha_step_up_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GachaStepUpUserInfo>> {
    sqlx::query_as::<_, GachaStepUpUserInfo>("SELECT * FROM GachaStepUpUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GachaStepUpUserInfo rows for a UID.
pub async fn delete_gacha_step_up_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GachaStepUpUserInfo WHERE Uid = ?")
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
) -> sqlx::Result<GachaStepUpUserInfo> {
    sqlx::query_as::<_, GachaStepUpUserInfo>(
        "SELECT * FROM GachaStepUpUserInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GachaStepUpUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GachaStepUpUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GachaStepUpUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GachaStepUpUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GachaStepUpUserInfo (
    Uid,
    GroupId,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_id)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
