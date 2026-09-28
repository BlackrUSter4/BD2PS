use crate::models::game::friend::friend_info::FriendInfo;
use sqlx::SqlitePool;

/// Add a single FriendInfo record from a Rust struct.
pub async fn add_friend_info(pool: &SqlitePool, data: &FriendInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FriendInfo (
    Uid,
    OwnerIndex,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    UserId,
    TitleId,
    Date,
    GuildBaseInfoIndex,
    LastLoginDate,
    SupporterInfoIndex,
    Status
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.user_id)
    .bind(&data.title_id)
    .bind(&data.date)
    .bind(&data.guild_base_info_index)
    .bind(&data.last_login_date)
    .bind(&data.supporter_info_index)
    .bind(&data.status)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_friend_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FriendInfo>> {
    sqlx::query_as::<_, FriendInfo>("SELECT * FROM FriendInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Get the row for one relationship (this account's view of `owner_index`), if any.
pub async fn get_by_uid_and_owner(
    pool: &SqlitePool,
    uid: i64,
    owner_index: i64,
) -> sqlx::Result<Option<FriendInfo>> {
    sqlx::query_as::<_, FriendInfo>("SELECT * FROM FriendInfo WHERE Uid = ? AND OwnerIndex = ?")
        .bind(uid)
        .bind(owner_index)
        .fetch_optional(pool)
        .await
}

/// Fetch this account's rows filtered by relationship status (0=confirmed, 1=pending
/// sent, 2=pending received).
pub async fn get_by_status(
    pool: &SqlitePool,
    uid: i64,
    status: i32,
) -> sqlx::Result<Vec<FriendInfo>> {
    sqlx::query_as::<_, FriendInfo>("SELECT * FROM FriendInfo WHERE Uid = ? AND Status = ?")
        .bind(uid)
        .bind(status)
        .fetch_all(pool)
        .await
}

/// Update just the Status of one relationship row.
pub async fn update_status(
    pool: &SqlitePool,
    uid: i64,
    owner_index: i64,
    status: i32,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE FriendInfo SET Status = ? WHERE Uid = ? AND OwnerIndex = ?")
        .bind(status)
        .bind(uid)
        .bind(owner_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete one relationship row (both directions are deleted independently by the caller).
pub async fn delete_by_uid_and_owner(
    pool: &SqlitePool,
    uid: i64,
    owner_index: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FriendInfo WHERE Uid = ? AND OwnerIndex = ?")
        .bind(uid)
        .bind(owner_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Real accounts on this server that aren't this account and aren't already in any
/// relationship with it (confirmed or pending either direction) — used for recommendations.
pub async fn list_recommendable(
    pool: &SqlitePool,
    uid: i64,
    limit: i64,
) -> sqlx::Result<Vec<i64>> {
    let rows: Vec<(i64,)> = sqlx::query_as(
        r#"
SELECT Uid FROM Account
WHERE Uid != ?
AND Uid NOT IN (SELECT OwnerIndex FROM FriendInfo WHERE Uid = ?)
LIMIT ?
"#,
    )
    .bind(uid)
    .bind(uid)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(u,)| u).collect())
}

/// Delete all FriendInfo rows for a UID.
pub async fn delete_friend_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FriendInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<FriendInfo> {
    sqlx::query_as::<_, FriendInfo>("SELECT * FROM FriendInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<FriendInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM FriendInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, FriendInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &FriendInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO FriendInfo (
    Uid,
    OwnerIndex,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    UserId,
    TitleId,
    Date,
    GuildBaseInfoIndex,
    LastLoginDate,
    SupporterInfoIndex,
    Status
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.user_id)
    .bind(&data.title_id)
    .bind(&data.date)
    .bind(&data.guild_base_info_index)
    .bind(&data.last_login_date)
    .bind(&data.supporter_info_index)
    .bind(&data.status)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
