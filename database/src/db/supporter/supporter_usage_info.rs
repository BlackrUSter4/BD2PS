use crate::models::game::supporter::supporter_usage_info::SupporterUsageInfo;
use sqlx::SqlitePool;

/// Add a single SupporterUsageInfo record from a Rust struct.
pub async fn add_supporter_usage_info(
    pool: &SqlitePool,
    data: &SupporterUsageInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO SupporterUsageInfo (
    Uid,
    Id,
    BorrowerOwnerIndex,
    BorrowerUserId,
    BorrowerPortraitCostumeId,
    BorrowerPortraitDesignId,
    BorrowerTitleId,
    SupporterOwnerIndex,
    SupporterSlotIndex,
    SupporterCostumeId,
    SupporterDesignId,
    BorrowType,
    RewardReceived,
    UseDate
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.id)
    .bind(&data.borrower_owner_index)
    .bind(&data.borrower_user_id)
    .bind(&data.borrower_portrait_costume_id)
    .bind(&data.borrower_portrait_design_id)
    .bind(&data.borrower_title_id)
    .bind(&data.supporter_owner_index)
    .bind(&data.supporter_slot_index)
    .bind(&data.supporter_costume_id)
    .bind(&data.supporter_design_id)
    .bind(&data.borrow_type)
    .bind(&data.reward_received)
    .bind(&data.use_date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_supporter_usage_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<SupporterUsageInfo>> {
    sqlx::query_as::<_, SupporterUsageInfo>("SELECT * FROM SupporterUsageInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Record a new borrow event, using the row's own auto-generated Index as its client-
/// facing Id (the table has both — Id is what SupporterRewardRequest references).
#[allow(clippy::too_many_arguments)]
pub async fn record_usage(
    pool: &SqlitePool,
    uid: i64,
    borrower_owner_index: i64,
    borrower_user_id: &str,
    supporter_owner_index: i64,
    supporter_slot_index: i32,
    supporter_costume_id: Option<i32>,
    borrow_type: i32,
) -> sqlx::Result<i64> {
    let index = insert(
        pool,
        &SupporterUsageInfo {
            index: 0,
            uid,
            id: None,
            borrower_owner_index: Some(borrower_owner_index),
            borrower_user_id: Some(borrower_user_id.to_string()),
            borrower_portrait_costume_id: None,
            borrower_portrait_design_id: None,
            borrower_title_id: None,
            supporter_owner_index: Some(supporter_owner_index),
            supporter_slot_index: Some(supporter_slot_index),
            supporter_costume_id,
            supporter_design_id: None,
            borrow_type: Some(borrow_type),
            reward_received: Some(0),
            use_date: Some(chrono::Utc::now().timestamp_millis()),
        },
    )
    .await?;
    sqlx::query("UPDATE SupporterUsageInfo SET Id = ? WHERE Index = ?")
        .bind(index)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(index)
}

/// Get this account's rental-history rows (as borrower), most recent first, optionally
/// paginated from `last_id`.
pub async fn get_history(
    pool: &SqlitePool,
    uid: i64,
    last_id: Option<i64>,
    limit: i32,
) -> sqlx::Result<Vec<SupporterUsageInfo>> {
    match last_id {
        Some(id) => {
            sqlx::query_as::<_, SupporterUsageInfo>(
                "SELECT * FROM SupporterUsageInfo WHERE Uid = ? AND Id < ? ORDER BY Id DESC LIMIT ?",
            )
            .bind(uid)
            .bind(id)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
        None => {
            sqlx::query_as::<_, SupporterUsageInfo>(
                "SELECT * FROM SupporterUsageInfo WHERE Uid = ? ORDER BY Id DESC LIMIT ?",
            )
            .bind(uid)
            .bind(limit)
            .fetch_all(pool)
            .await
        }
    }
}

/// Fetch specific usage rows by id (for reward claiming), scoped to this account.
pub async fn get_by_ids(
    pool: &SqlitePool,
    uid: i64,
    ids: &[i64],
) -> sqlx::Result<Vec<SupporterUsageInfo>> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT * FROM SupporterUsageInfo WHERE Uid = ? AND Id IN ({})", placeholders);
    let mut q = sqlx::query_as::<_, SupporterUsageInfo>(&query).bind(uid);
    for id in ids {
        q = q.bind(id);
    }
    q.fetch_all(pool).await
}

/// Mark a usage row's reward as received.
pub async fn mark_reward_received(pool: &SqlitePool, uid: i64, id: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE SupporterUsageInfo SET RewardReceived = 1 WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// How many reward-received usages this account has claimed today (real UTC calendar day).
pub async fn count_rewards_claimed_today(pool: &SqlitePool, uid: i64) -> sqlx::Result<i64> {
    let day_start = chrono::Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis();
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM SupporterUsageInfo WHERE Uid = ? AND RewardReceived = 1 AND UseDate >= ?",
    )
    .bind(uid)
    .bind(day_start)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Delete all SupporterUsageInfo rows for a UID.
pub async fn delete_supporter_usage_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM SupporterUsageInfo WHERE Uid = ?")
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
) -> sqlx::Result<SupporterUsageInfo> {
    sqlx::query_as::<_, SupporterUsageInfo>(
        "SELECT * FROM SupporterUsageInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<SupporterUsageInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM SupporterUsageInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, SupporterUsageInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &SupporterUsageInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO SupporterUsageInfo (
    Uid,
    Id,
    BorrowerOwnerIndex,
    BorrowerUserId,
    BorrowerPortraitCostumeId,
    BorrowerPortraitDesignId,
    BorrowerTitleId,
    SupporterOwnerIndex,
    SupporterSlotIndex,
    SupporterCostumeId,
    SupporterDesignId,
    BorrowType,
    RewardReceived,
    UseDate
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.id)
    .bind(&data.borrower_owner_index)
    .bind(&data.borrower_user_id)
    .bind(&data.borrower_portrait_costume_id)
    .bind(&data.borrower_portrait_design_id)
    .bind(&data.borrower_title_id)
    .bind(&data.supporter_owner_index)
    .bind(&data.supporter_slot_index)
    .bind(&data.supporter_costume_id)
    .bind(&data.supporter_design_id)
    .bind(&data.borrow_type)
    .bind(&data.reward_received)
    .bind(&data.use_date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
