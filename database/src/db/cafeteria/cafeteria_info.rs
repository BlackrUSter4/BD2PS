use crate::models::game::cafeteria::cafeteria_info::CafeteriaInfo;
use sqlx::SqlitePool;

/// Add a single CafeteriaInfo record from a Rust struct.
pub async fn add_cafeteria_info(pool: &SqlitePool, data: &CafeteriaInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CafeteriaInfo (
    Uid,
    Level,
    RewardReceiptTime,
    SpawnTime,
    OngoingManageId,
    DailyRegularCostumeIds,
    RewardedDailyRegularCostumeIds,
    DailyConnectionCostumeId,
    CanGetPhoneNumber,
    DailyNpcRewardCurrencyCount,
    IntroStoryRewardClaimed
) VALUES (
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.level)
    .bind(&data.reward_receipt_time)
    .bind(&data.spawn_time)
    .bind(&data.ongoing_manage_id)
    .bind(&data.daily_regular_costume_ids)
    .bind(&data.rewarded_daily_regular_costume_ids)
    .bind(&data.daily_connection_costume_id)
    .bind(&data.can_get_phone_number)
    .bind(&data.daily_npc_reward_currency_count)
    .bind(&data.intro_story_reward_claimed)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_cafeteria_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CafeteriaInfo>> {
    sqlx::query_as::<_, CafeteriaInfo>("SELECT * FROM CafeteriaInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the account's single CafeteriaInfo row, creating a fresh one (level 1, everything else
/// unset) if this account has never opened the Cafeteria before.
pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<CafeteriaInfo> {
    if let Some(row) = get_cafeteria_info(pool, uid).await?.into_iter().next() {
        return Ok(row);
    }

    let fresh = CafeteriaInfo {
        index: 0,
        uid,
        level: Some(1),
        reward_receipt_time: None,
        spawn_time: None,
        ongoing_manage_id: Some(0),
        daily_regular_costume_ids: None,
        rewarded_daily_regular_costume_ids: None,
        daily_connection_costume_id: None,
        can_get_phone_number: Some(0),
        daily_npc_reward_currency_count: Some(0),
        intro_story_reward_claimed: 0,
    };
    add_cafeteria_info(pool, &fresh).await?;
    Ok(get_cafeteria_info(pool, uid)
        .await?
        .into_iter()
        .next()
        .expect("row was just inserted"))
}

/// Full-row update, matched by Uid (there is exactly one CafeteriaInfo row per account).
pub async fn update_cafeteria_info(pool: &SqlitePool, data: &CafeteriaInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
UPDATE CafeteriaInfo SET
    Level = ?,
    RewardReceiptTime = ?,
    SpawnTime = ?,
    OngoingManageId = ?,
    DailyRegularCostumeIds = ?,
    RewardedDailyRegularCostumeIds = ?,
    DailyConnectionCostumeId = ?,
    CanGetPhoneNumber = ?,
    DailyNpcRewardCurrencyCount = ?,
    IntroStoryRewardClaimed = ?
WHERE Uid = ?
"#,
    )
    .bind(&data.level)
    .bind(&data.reward_receipt_time)
    .bind(&data.spawn_time)
    .bind(&data.ongoing_manage_id)
    .bind(&data.daily_regular_costume_ids)
    .bind(&data.rewarded_daily_regular_costume_ids)
    .bind(&data.daily_connection_costume_id)
    .bind(&data.can_get_phone_number)
    .bind(&data.daily_npc_reward_currency_count)
    .bind(&data.intro_story_reward_claimed)
    .bind(&data.uid)
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete all CafeteriaInfo rows for a UID.
pub async fn delete_cafeteria_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CafeteriaInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<CafeteriaInfo> {
    sqlx::query_as::<_, CafeteriaInfo>("SELECT * FROM CafeteriaInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<CafeteriaInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM CafeteriaInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, CafeteriaInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &CafeteriaInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO CafeteriaInfo (
    Uid,
    Level,
    RewardReceiptTime,
    SpawnTime,
    OngoingManageId,
    DailyRegularCostumeIds,
    RewardedDailyRegularCostumeIds,
    DailyConnectionCostumeId,
    CanGetPhoneNumber,
    DailyNpcRewardCurrencyCount,
    IntroStoryRewardClaimed
) VALUES (
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.level)
    .bind(&data.reward_receipt_time)
    .bind(&data.spawn_time)
    .bind(&data.ongoing_manage_id)
    .bind(&data.daily_regular_costume_ids)
    .bind(&data.rewarded_daily_regular_costume_ids)
    .bind(&data.daily_connection_costume_id)
    .bind(&data.can_get_phone_number)
    .bind(&data.daily_npc_reward_currency_count)
    .bind(&data.intro_story_reward_claimed)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
