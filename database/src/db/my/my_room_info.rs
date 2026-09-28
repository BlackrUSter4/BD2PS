use crate::models::game::my::my_room_info::MyRoomInfo;
use sqlx::SqlitePool;

/// Add a single MyRoomInfo record from a Rust struct.
pub async fn add_my_room_info(pool: &SqlitePool, data: &MyRoomInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomInfo (
    Uid,
    Id,
    Name,
    IsHidden
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.is_hidden)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_room_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MyRoomInfo>> {
    sqlx::query_as::<_, MyRoomInfo>("SELECT * FROM MyRoomInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MyRoomInfo rows for a UID.
pub async fn delete_my_room_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MyRoomInfo> {
    sqlx::query_as::<_, MyRoomInfo>("SELECT * FROM MyRoomInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MyRoomInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MyRoomInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MyRoomInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MyRoomInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MyRoomInfo (
    Uid,
    Id,
    Name,
    IsHidden
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.name)
    .bind(&data.is_hidden)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Get a single room by its game-facing room Id (not the DB rowid).
pub async fn get_by_uid_and_id(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
) -> sqlx::Result<Option<MyRoomInfo>> {
    sqlx::query_as::<_, MyRoomInfo>("SELECT * FROM MyRoomInfo WHERE Uid = ? AND Id = ?")
        .bind(uid)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Update a room's name and/or hidden flag.
pub async fn update_name_hidden(
    pool: &SqlitePool,
    uid: i64,
    id: i32,
    name: Option<&str>,
    is_hidden: Option<i32>,
) -> sqlx::Result<()> {
    if let Some(name) = name {
        sqlx::query("UPDATE MyRoomInfo SET Name = ? WHERE Uid = ? AND Id = ?")
            .bind(name)
            .bind(uid)
            .bind(id)
            .execute(pool)
            .await?;
    }
    if let Some(is_hidden) = is_hidden {
        sqlx::query("UPDATE MyRoomInfo SET IsHidden = ? WHERE Uid = ? AND Id = ?")
            .bind(is_hidden)
            .bind(uid)
            .bind(id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
