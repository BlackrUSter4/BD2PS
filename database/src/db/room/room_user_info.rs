use crate::models::game::room::room_user_info::RoomUserInfo;
use sqlx::SqlitePool;

/// Add a single RoomUserInfo record from a Rust struct.
pub async fn add_room_user_info(pool: &SqlitePool, data: &RoomUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RoomUserInfo (
    Uid,
    OwnerIndex,
    Ip,
    Port,
    NatType
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
    .bind(&data.owner_index)
    .bind(&data.ip)
    .bind(&data.port)
    .bind(&data.nat_type)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_room_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<RoomUserInfo>> {
    sqlx::query_as::<_, RoomUserInfo>("SELECT * FROM RoomUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RoomUserInfo rows for a UID.
pub async fn delete_room_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RoomUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<RoomUserInfo> {
    sqlx::query_as::<_, RoomUserInfo>("SELECT * FROM RoomUserInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<RoomUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM RoomUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, RoomUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &RoomUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO RoomUserInfo (
    Uid,
    OwnerIndex,
    Ip,
    Port,
    NatType
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
    .bind(&data.owner_index)
    .bind(&data.ip)
    .bind(&data.port)
    .bind(&data.nat_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
