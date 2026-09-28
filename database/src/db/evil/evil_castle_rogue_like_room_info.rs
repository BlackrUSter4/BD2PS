use crate::models::game::evil::evil_castle_rogue_like_room_info::EvilCastleRogueLikeRoomInfo;
use sqlx::SqlitePool;

/// Add a single EvilCastleRogueLikeRoomInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_room_info(
    pool: &SqlitePool,
    data: &EvilCastleRogueLikeRoomInfo,
) -> sqlx::Result<()> {
    insert(pool, data).await.map(|_| ())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_room_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EvilCastleRogueLikeRoomInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeRoomInfo>(
        "SELECT * FROM EvilCastleRogueLikeRoomInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Fetch all rooms for a given UID + floor.
pub async fn get_by_floor(
    pool: &SqlitePool,
    uid: i64,
    floor: i32,
) -> sqlx::Result<Vec<EvilCastleRogueLikeRoomInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeRoomInfo>(
        "SELECT * FROM EvilCastleRogueLikeRoomInfo WHERE Uid = ? AND Floor = ? ORDER BY Number",
    )
    .bind(uid)
    .bind(floor)
    .fetch_all(pool)
    .await
}

/// Mark a specific room cleared.
pub async fn set_clear(
    pool: &SqlitePool,
    uid: i64,
    floor: i32,
    number: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE EvilCastleRogueLikeRoomInfo SET IsClear = 1 WHERE Uid = ? AND Floor = ? AND Number = ?",
    )
    .bind(uid)
    .bind(floor)
    .bind(number)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete all EvilCastleRogueLikeRoomInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_room_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeRoomInfo WHERE Uid = ?")
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
) -> sqlx::Result<EvilCastleRogueLikeRoomInfo> {
    sqlx::query_as::<_, EvilCastleRogueLikeRoomInfo>(
        "SELECT * FROM EvilCastleRogueLikeRoomInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<EvilCastleRogueLikeRoomInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM EvilCastleRogueLikeRoomInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, EvilCastleRogueLikeRoomInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EvilCastleRogueLikeRoomInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeRoomInfo (
    Uid,
    Floor,
    Number,
    GroupId,
    Id,
    IsClear
) VALUES (
    ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.floor)
    .bind(&data.number)
    .bind(&data.group_id)
    .bind(&data.id)
    .bind(&data.is_clear)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
