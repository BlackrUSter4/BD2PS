use crate::models::game::server::server_info::ServerInfo;
use sqlx::SqlitePool;

/// Add a single ServerInfo record from a Rust struct.
pub async fn add_server_info(pool: &SqlitePool, data: &ServerInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ServerInfo (
    Uid,
    Region,
    GameServerInfo,
    CdnInfo,
    OpenFlag,
    LogServerInfo,
    CharServerInfo,
    CouponWebInfo,
    GameDataInfo,
    GameDataVersion
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.region)
    .bind(&data.game_server_info)
    .bind(&data.cdn_info)
    .bind(&data.open_flag)
    .bind(&data.log_server_info)
    .bind(&data.char_server_info)
    .bind(&data.coupon_web_info)
    .bind(&data.game_data_info)
    .bind(&data.game_data_version)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_server_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<ServerInfo>> {
    sqlx::query_as::<_, ServerInfo>("SELECT * FROM ServerInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ServerInfo rows for a UID.
pub async fn delete_server_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ServerInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ServerInfo> {
    sqlx::query_as::<_, ServerInfo>("SELECT * FROM ServerInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ServerInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ServerInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ServerInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ServerInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ServerInfo (
    Uid,
    Region,
    GameServerInfo,
    CdnInfo,
    OpenFlag,
    LogServerInfo,
    CharServerInfo,
    CouponWebInfo,
    GameDataInfo,
    GameDataVersion
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.region)
    .bind(&data.game_server_info)
    .bind(&data.cdn_info)
    .bind(&data.open_flag)
    .bind(&data.log_server_info)
    .bind(&data.char_server_info)
    .bind(&data.coupon_web_info)
    .bind(&data.game_data_info)
    .bind(&data.game_data_version)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
