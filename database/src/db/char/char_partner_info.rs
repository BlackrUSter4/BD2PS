use crate::models::game::char::char_partner_info::CharPartnerInfo;
use sqlx::SqlitePool;

/// Add a single CharPartnerInfo record from a Rust struct.
pub async fn add_char_partner_info(pool: &SqlitePool, data: &CharPartnerInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CharPartnerInfo (
    Uid,
    MainUniqueId,
    SubUniqueId,
    Point,
    Reward
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
    .bind(&data.main_unique_id)
    .bind(&data.sub_unique_id)
    .bind(&data.point)
    .bind(&data.reward)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_char_partner_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<CharPartnerInfo>> {
    sqlx::query_as::<_, CharPartnerInfo>("SELECT * FROM CharPartnerInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CharPartnerInfo rows for a UID.
pub async fn delete_char_partner_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharPartnerInfo WHERE Uid = ?")
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
) -> sqlx::Result<CharPartnerInfo> {
    sqlx::query_as::<_, CharPartnerInfo>(
        "SELECT * FROM CharPartnerInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<CharPartnerInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CharPartnerInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CharPartnerInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Get a single partner pairing row by main+sub unique ids.
pub async fn get_by_pair(
    pool: &SqlitePool,
    uid: i64,
    main_unique_id: i32,
    sub_unique_id: i32,
) -> sqlx::Result<Option<CharPartnerInfo>> {
    sqlx::query_as::<_, CharPartnerInfo>(
        "SELECT * FROM CharPartnerInfo WHERE Uid = ? AND MainUniqueId = ? AND SubUniqueId = ?",
    )
    .bind(uid)
    .bind(main_unique_id)
    .bind(sub_unique_id)
    .fetch_optional(pool)
    .await
}

/// Set the Reward bitmask for a pairing (claim tracking).
pub async fn set_reward_bits(
    pool: &SqlitePool,
    uid: i64,
    main_unique_id: i32,
    sub_unique_id: i32,
    reward: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE CharPartnerInfo SET Reward = ? WHERE Uid = ? AND MainUniqueId = ? AND SubUniqueId = ?",
    )
    .bind(reward)
    .bind(uid)
    .bind(main_unique_id)
    .bind(sub_unique_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CharPartnerInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CharPartnerInfo (
    Uid,
    MainUniqueId,
    SubUniqueId,
    Point,
    Reward
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
    .bind(&data.main_unique_id)
    .bind(&data.sub_unique_id)
    .bind(&data.point)
    .bind(&data.reward)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
