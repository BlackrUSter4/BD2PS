use crate::models::game::pass::pass_reward_info::PassRewardInfo;
use sqlx::SqlitePool;

/// Add a single PassRewardInfo record from a Rust struct.
pub async fn add_pass_reward_info(pool: &SqlitePool, data: &PassRewardInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO PassRewardInfo (
    Uid,
    PassId,
    Id,
    Basic,
    Premium1
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pass_id)
    .bind(&data.id)
    .bind(&data.basic)
    .bind(&data.premium_1)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_pass_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<PassRewardInfo>> {
    sqlx::query_as::<_, PassRewardInfo>("SELECT * FROM PassRewardInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_pass_and_id(pool: &SqlitePool, uid: i64, pass_id: i32, id: i32) -> sqlx::Result<Option<PassRewardInfo>> {
    sqlx::query_as::<_, PassRewardInfo>("SELECT * FROM PassRewardInfo WHERE Uid = ? AND PassId = ? AND Id = ?")
        .bind(uid)
        .bind(pass_id)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn mark_claimed(pool: &SqlitePool, uid: i64, pass_id: i32, id: i32, basic: bool, premium: bool) -> sqlx::Result<()> {
    if let Some(row) = get_by_pass_and_id(pool, uid, pass_id, id).await? {
        sqlx::query("UPDATE PassRewardInfo SET Basic = ?, Premium1 = ? WHERE \"Index\" = ?")
            .bind(basic || row.basic.unwrap_or(false))
            .bind(premium || row.premium_1.unwrap_or(false))
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_pass_reward_info(
            pool,
            &PassRewardInfo { index: 0, uid, pass_id: Some(pass_id), id: Some(id), basic: Some(basic), premium_1: Some(premium) },
        )
        .await?;
    }
    Ok(())
}

/// Delete all PassRewardInfo rows for a UID.
pub async fn delete_pass_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM PassRewardInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<PassRewardInfo> {
    sqlx::query_as::<_, PassRewardInfo>("SELECT * FROM PassRewardInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<PassRewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM PassRewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, PassRewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &PassRewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO PassRewardInfo (
    Uid,
    PassId,
    Id,
    Basic,
    Premium1
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pass_id)
    .bind(&data.id)
    .bind(&data.basic)
    .bind(&data.premium_1)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
