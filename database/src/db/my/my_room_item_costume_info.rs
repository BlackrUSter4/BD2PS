use crate::models::game::my::my_room_item_costume_info::MyRoomItemCostumeInfo;
use sqlx::SqlitePool;

/// Add a single MyRoomItemCostumeInfo record from a Rust struct.
pub async fn add_my_room_item_costume_info(
    pool: &SqlitePool,
    data: &MyRoomItemCostumeInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomItemCostumeInfo (
    Uid,
    InvenIndex,
    Id,
    UseMyRoomCount
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.use_my_room_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_room_item_costume_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MyRoomItemCostumeInfo>> {
    sqlx::query_as::<_, MyRoomItemCostumeInfo>("SELECT * FROM MyRoomItemCostumeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MyRoomItemCostumeInfo rows for a UID.
pub async fn delete_my_room_item_costume_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomItemCostumeInfo WHERE Uid = ?")
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
) -> sqlx::Result<MyRoomItemCostumeInfo> {
    sqlx::query_as::<_, MyRoomItemCostumeInfo>(
        "SELECT * FROM MyRoomItemCostumeInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MyRoomItemCostumeInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MyRoomItemCostumeInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MyRoomItemCostumeInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MyRoomItemCostumeInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MyRoomItemCostumeInfo (
    Uid,
    InvenIndex,
    Id,
    UseMyRoomCount
) VALUES (
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.inven_index)
    .bind(&data.id)
    .bind(&data.use_my_room_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
