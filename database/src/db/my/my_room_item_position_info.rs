use crate::models::game::my::my_room_item_position_info::MyRoomItemPositionInfo;
use sqlx::SqlitePool;

/// Add a single MyRoomItemPositionInfo record from a Rust struct.
pub async fn add_my_room_item_position_info(
    pool: &SqlitePool,
    data: &MyRoomItemPositionInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomItemPositionInfo (
    Uid,
    InvenIndex,
    ObjectType,
    PositionType,
    X,
    Y,
    Rotate,
    Interact,
    ItemAnimation,
    IsWallHidden,
    RoomId
) VALUES (
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.object_type)
    .bind(&data.position_type)
    .bind(&data.x)
    .bind(&data.y)
    .bind(&data.rotate)
    .bind(&data.interact)
    .bind(&data.item_animation)
    .bind(&data.is_wall_hidden)
    .bind(&data.room_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_room_item_position_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MyRoomItemPositionInfo>> {
    sqlx::query_as::<_, MyRoomItemPositionInfo>(
        "SELECT * FROM MyRoomItemPositionInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Fetch all position rows for a given UID scoped to one room id.
pub async fn get_by_uid_and_room(
    pool: &SqlitePool,
    uid: i64,
    room_id: i32,
) -> sqlx::Result<Vec<MyRoomItemPositionInfo>> {
    sqlx::query_as::<_, MyRoomItemPositionInfo>(
        "SELECT * FROM MyRoomItemPositionInfo WHERE Uid = ? AND RoomId = ?",
    )
    .bind(uid)
    .bind(room_id)
    .fetch_all(pool)
    .await
}

/// Delete all MyRoomItemPositionInfo rows for a UID.
pub async fn delete_my_room_item_position_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomItemPositionInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all position rows for a UID scoped to one room id (used before re-saving a room layout).
pub async fn delete_by_uid_and_room(pool: &SqlitePool, uid: i64, room_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomItemPositionInfo WHERE Uid = ? AND RoomId = ?")
        .bind(uid)
        .bind(room_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<MyRoomItemPositionInfo> {
    sqlx::query_as::<_, MyRoomItemPositionInfo>(
        "SELECT * FROM MyRoomItemPositionInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MyRoomItemPositionInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MyRoomItemPositionInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MyRoomItemPositionInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MyRoomItemPositionInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MyRoomItemPositionInfo (
    Uid,
    InvenIndex,
    ObjectType,
    PositionType,
    X,
    Y,
    Rotate,
    Interact,
    ItemAnimation,
    IsWallHidden,
    RoomId
) VALUES (
    ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.object_type)
    .bind(&data.position_type)
    .bind(&data.x)
    .bind(&data.y)
    .bind(&data.rotate)
    .bind(&data.interact)
    .bind(&data.item_animation)
    .bind(&data.is_wall_hidden)
    .bind(&data.room_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
