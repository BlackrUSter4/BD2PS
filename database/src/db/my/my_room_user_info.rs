use crate::models::game::my::my_room_user_info::MyRoomUserInfo;
use sqlx::SqlitePool;

/// Add a single MyRoomUserInfo record from a Rust struct.
pub async fn add_my_room_user_info(pool: &SqlitePool, data: &MyRoomUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PrimaryMyRoomId,
    ItemInfoIndex,
    CostumeInfoIndex,
    TrophyInfoIndex,
    MyRoomIndex,
    MyRoomLikeCount,
    MyRoomLikeDate,
    PortraitCostumeDesignId
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.primary_my_room_id)
    .bind(&data.item_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.trophy_info_index)
    .bind(&data.my_room_index)
    .bind(&data.my_room_like_count)
    .bind(&data.my_room_like_date)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_room_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MyRoomUserInfo>> {
    sqlx::query_as::<_, MyRoomUserInfo>("SELECT * FROM MyRoomUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MyRoomUserInfo rows for a UID.
pub async fn delete_my_room_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MyRoomUserInfo> {
    sqlx::query_as::<_, MyRoomUserInfo>("SELECT * FROM MyRoomUserInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MyRoomUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MyRoomUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MyRoomUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MyRoomUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MyRoomUserInfo (
    Uid,
    OwnerIndex,
    UserId,
    PortraitCostumeId,
    PrimaryMyRoomId,
    ItemInfoIndex,
    CostumeInfoIndex,
    TrophyInfoIndex,
    MyRoomIndex,
    MyRoomLikeCount,
    MyRoomLikeDate,
    PortraitCostumeDesignId
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
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.primary_my_room_id)
    .bind(&data.item_info_index)
    .bind(&data.costume_info_index)
    .bind(&data.trophy_info_index)
    .bind(&data.my_room_index)
    .bind(&data.my_room_like_count)
    .bind(&data.my_room_like_date)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Fetch the single MyRoomUserInfo profile row for an account, if it exists.
pub async fn get_one(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<MyRoomUserInfo>> {
    sqlx::query_as::<_, MyRoomUserInfo>("SELECT * FROM MyRoomUserInfo WHERE Uid = ? LIMIT 1")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn update_primary_room(pool: &SqlitePool, uid: i64, primary_id: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE MyRoomUserInfo SET PrimaryMyRoomId = ? WHERE Uid = ?")
        .bind(primary_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_like(
    pool: &SqlitePool,
    uid: i64,
    like_count: i32,
    like_date: i64,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE MyRoomUserInfo SET MyRoomLikeCount = ?, MyRoomLikeDate = ? WHERE Uid = ?")
        .bind(like_count)
        .bind(like_date)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_share_option(pool: &SqlitePool, uid: i64, allow_scope_type: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE MyRoomUserInfo SET AllowScopeType = ? WHERE Uid = ?")
        .bind(allow_scope_type)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
