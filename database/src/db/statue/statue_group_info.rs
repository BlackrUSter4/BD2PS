use crate::models::game::statue::statue_group_info::StatueGroupInfo;
use sqlx::SqlitePool;

/// Add a single StatueGroupInfo record from a Rust struct.
pub async fn add_statue_group_info(pool: &SqlitePool, data: &StatueGroupInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO StatueGroupInfo (
    Uid,
    Id,
    Season,
    GroupRank,
    GuildBaseInfoIndex,
    UserStatueInfoIndex,
    ErrorFlag
) VALUES (
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
    .bind(&data.season)
    .bind(&data.group_rank)
    .bind(&data.guild_base_info_index)
    .bind(&data.user_statue_info_index)
    .bind(&data.error_flag)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_statue_group_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<StatueGroupInfo>> {
    sqlx::query_as::<_, StatueGroupInfo>("SELECT * FROM StatueGroupInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all StatueGroupInfo rows for a UID.
pub async fn delete_statue_group_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM StatueGroupInfo WHERE Uid = ?")
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
) -> sqlx::Result<StatueGroupInfo> {
    sqlx::query_as::<_, StatueGroupInfo>(
        "SELECT * FROM StatueGroupInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<StatueGroupInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM StatueGroupInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, StatueGroupInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &StatueGroupInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO StatueGroupInfo (
    Uid,
    Id,
    Season,
    GroupRank,
    GuildBaseInfoIndex,
    UserStatueInfoIndex,
    ErrorFlag
) VALUES (
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
    .bind(&data.season)
    .bind(&data.group_rank)
    .bind(&data.guild_base_info_index)
    .bind(&data.user_statue_info_index)
    .bind(&data.error_flag)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
