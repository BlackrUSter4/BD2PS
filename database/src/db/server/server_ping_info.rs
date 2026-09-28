use crate::models::game::server::server_ping_info::ServerPingInfo;
use sqlx::SqlitePool;

/// Add a single ServerPingInfo record from a Rust struct.
pub async fn add_server_ping_info(pool: &SqlitePool, data: &ServerPingInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ServerPingInfo (
    Uid,
    Region,
    Ping
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.region)
    .bind(&data.ping)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_server_ping_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ServerPingInfo>> {
    sqlx::query_as::<_, ServerPingInfo>("SELECT * FROM ServerPingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ServerPingInfo rows for a UID.
pub async fn delete_server_ping_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ServerPingInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ServerPingInfo> {
    sqlx::query_as::<_, ServerPingInfo>("SELECT * FROM ServerPingInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ServerPingInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ServerPingInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ServerPingInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ServerPingInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ServerPingInfo (
    Uid,
    Region,
    Ping
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.region)
    .bind(&data.ping)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
