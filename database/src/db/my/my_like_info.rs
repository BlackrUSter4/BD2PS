use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::my::my_like_info::MyLikeInfo;
/// Insert a full JSON array of MyLikeInfo records for a UID.
pub async fn insert_my_like_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("myLikeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_my_like_info: missing or invalid 'myLikeInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let target_owner_index = entry
            .get("targetOwnerIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        sqlx::query(
            r#"
INSERT INTO MyLikeInfo (
    Uid,
    TargetOwnerIndex
) VALUES (
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&target_owner_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MyLikeInfo record from a Rust struct.
pub async fn add_my_like_info(pool: &SqlitePool, data: &MyLikeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MyLikeInfo (
    Uid,
    TargetOwnerIndex
) VALUES (
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.target_owner_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_my_like_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MyLikeInfo>> {
    sqlx::query_as::<_, MyLikeInfo>("SELECT * FROM MyLikeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MyLikeInfo rows for a UID.
pub async fn delete_my_like_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MyLikeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Find an existing like from `uid` toward `target_owner_index`, if any.
pub async fn get_by_uid_and_target(
    pool: &SqlitePool,
    uid: i64,
    target_owner_index: i64,
) -> sqlx::Result<Option<MyLikeInfo>> {
    sqlx::query_as::<_, MyLikeInfo>(
        "SELECT * FROM MyLikeInfo WHERE Uid = ? AND TargetOwnerIndex = ?",
    )
    .bind(uid)
    .bind(target_owner_index)
    .fetch_optional(pool)
    .await
}

/// Record (or refresh the date of) a like from `uid` toward `target_owner_index`.
pub async fn upsert_like(pool: &SqlitePool, uid: i64, target_owner_index: i64, date: i64) -> sqlx::Result<()> {
    let existing = get_by_uid_and_target(pool, uid, target_owner_index).await?;
    match existing {
        Some(_) => {
            sqlx::query("UPDATE MyLikeInfo SET Date = ? WHERE Uid = ? AND TargetOwnerIndex = ?")
                .bind(date)
                .bind(uid)
                .bind(target_owner_index)
                .execute(pool)
                .await?;
        }
        None => {
            sqlx::query("INSERT INTO MyLikeInfo (Uid, TargetOwnerIndex, Date) VALUES (?, ?, ?)")
                .bind(uid)
                .bind(target_owner_index)
                .bind(date)
                .execute(pool)
                .await?;
        }
    }
    Ok(())
}

/// Total distinct likers for a target (all-time).
pub async fn count_for_target(pool: &SqlitePool, target_owner_index: i64) -> sqlx::Result<i64> {
    let (count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM MyLikeInfo WHERE TargetOwnerIndex = ?")
            .bind(target_owner_index)
            .fetch_one(pool)
            .await?;
    Ok(count)
}

/// Distinct likers for a target since a given timestamp (used for "this month").
pub async fn count_for_target_since(
    pool: &SqlitePool,
    target_owner_index: i64,
    since_date: i64,
) -> sqlx::Result<i64> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM MyLikeInfo WHERE TargetOwnerIndex = ? AND Date >= ?",
    )
    .bind(target_owner_index)
    .bind(since_date)
    .fetch_one(pool)
    .await?;
    Ok(count)
}