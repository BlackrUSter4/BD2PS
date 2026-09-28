use crate::models::game::my::my_room_shop_info::MyRoomShopInfo;
use sqlx::SqlitePool;

/// Add a single MyRoomShopInfo record from a Rust struct.
pub async fn add_my_room_shop_info(pool: &SqlitePool, data: &MyRoomShopInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyRoomShopInfo (
    Uid,
    Id,
    BuyCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.buy_count)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_room_shop_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MyRoomShopInfo>> {
    sqlx::query_as::<_, MyRoomShopInfo>("SELECT * FROM MyRoomShopInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MyRoomShopInfo rows for a UID.
pub async fn delete_my_room_shop_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyRoomShopInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MyRoomShopInfo> {
    sqlx::query_as::<_, MyRoomShopInfo>("SELECT * FROM MyRoomShopInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MyRoomShopInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MyRoomShopInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MyRoomShopInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MyRoomShopInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MyRoomShopInfo (
    Uid,
    Id,
    BuyCount
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.id)
    .bind(&data.buy_count)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Increment (or create) the buy-count row for a given shop entry id.
pub async fn increment_buy_count(pool: &SqlitePool, uid: i64, id: i32, amount: i32) -> sqlx::Result<()> {
    let existing = sqlx::query_as::<_, MyRoomShopInfo>(
        "SELECT * FROM MyRoomShopInfo WHERE Uid = ? AND Id = ?",
    )
    .bind(uid)
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match existing {
        Some(row) => {
            sqlx::query("UPDATE MyRoomShopInfo SET BuyCount = BuyCount + ? WHERE Uid = ? AND Id = ?")
                .bind(amount)
                .bind(uid)
                .bind(id)
                .execute(pool)
                .await?;
            let _ = row;
        }
        None => {
            sqlx::query("INSERT INTO MyRoomShopInfo (Uid, Id, BuyCount) VALUES (?, ?, ?)")
                .bind(uid)
                .bind(id)
                .bind(amount)
                .execute(pool)
                .await?;
        }
    }
    Ok(())
}
