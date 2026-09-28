use crate::models::game::guild::guild_member_info::GuildMemberInfo;
use sqlx::SqlitePool;

/// Add a single GuildMemberInfo record from a Rust struct.
pub async fn add_guild_member_info(pool: &SqlitePool, data: &GuildMemberInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildMemberInfo (
    Uid,
    Id,
    OwnerIndex,
    UserId,
    TitleId,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    Role,
    Point,
    SupporterInfoIndex,
    LastLoginDate,
    UpdateDate,
    Date
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.title_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.role)
    .bind(&data.point)
    .bind(&data.supporter_info_index)
    .bind(&data.last_login_date)
    .bind(&data.update_date)
    .bind(&data.date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_member_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<GuildMemberInfo>> {
    sqlx::query_as::<_, GuildMemberInfo>("SELECT * FROM GuildMemberInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildMemberInfo rows for a UID.
pub async fn delete_guild_member_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildMemberInfo WHERE Uid = ?")
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
) -> sqlx::Result<GuildMemberInfo> {
    sqlx::query_as::<_, GuildMemberInfo>(
        "SELECT * FROM GuildMemberInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<GuildMemberInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM GuildMemberInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, GuildMemberInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// All roster rows tagged with a given guild id, across every account's own
/// copy (each member's client keeps a locally-synced roster view).
pub async fn get_by_guild_id_any_uid(pool: &SqlitePool, guild_id: i64) -> sqlx::Result<Vec<GuildMemberInfo>> {
    sqlx::query_as::<_, GuildMemberInfo>("SELECT * FROM GuildMemberInfo WHERE Id = ?")
        .bind(guild_id)
        .fetch_all(pool)
        .await
}

/// Delete this member's roster row from every account's synced copy (used on
/// leave/ban so nobody's roster view keeps a stale entry).
pub async fn delete_member_everywhere(pool: &SqlitePool, guild_id: i64, member_owner_index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildMemberInfo WHERE Id = ? AND OwnerIndex = ?")
        .bind(guild_id)
        .bind(member_owner_index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_role(pool: &SqlitePool, guild_id: i64, member_owner_index: i64, role: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE GuildMemberInfo SET Role = ? WHERE Id = ? AND OwnerIndex = ?")
        .bind(serde_json::json!(role))
        .bind(guild_id)
        .bind(member_owner_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &GuildMemberInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO GuildMemberInfo (
    Uid,
    Id,
    OwnerIndex,
    UserId,
    TitleId,
    PortraitCostumeId,
    PortraitCostumeDesignId,
    Role,
    Point,
    SupporterInfoIndex,
    LastLoginDate,
    UpdateDate,
    Date
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.title_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.role)
    .bind(&data.point)
    .bind(&data.supporter_info_index)
    .bind(&data.last_login_date)
    .bind(&data.update_date)
    .bind(&data.date)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
