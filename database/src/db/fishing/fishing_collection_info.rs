use crate::models::game::fishing::fishing_collection_info::FishingCollectionInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<FishingCollectionInfo>> {
    sqlx::query_as::<_, FishingCollectionInfo>("SELECT * FROM FishingCollectionInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Record a catch against the personal-best collection log for this fish species, widening
/// MinSize/MaxSize as needed. Returns the (possibly just-created) row.
pub async fn record_catch(
    pool: &SqlitePool,
    uid: i64,
    fish_id: i32,
    size: i32,
    now: i64,
) -> sqlx::Result<FishingCollectionInfo> {
    let existing = sqlx::query_as::<_, FishingCollectionInfo>(
        "SELECT * FROM FishingCollectionInfo WHERE Uid = ? AND FishId = ?",
    )
    .bind(uid)
    .bind(fish_id)
    .fetch_optional(pool)
    .await?;

    let row = match existing {
        Some(row) => {
            let max_size = row.max_size.max(size);
            let min_size = row.min_size.min(size);
            sqlx::query(
                "UPDATE FishingCollectionInfo SET MaxSize = ?, MinSize = ? WHERE Uid = ? AND FishId = ?",
            )
            .bind(max_size)
            .bind(min_size)
            .bind(uid)
            .bind(fish_id)
            .execute(pool)
            .await?;
            FishingCollectionInfo { max_size, min_size, ..row }
        }
        None => {
            sqlx::query(
                "INSERT INTO FishingCollectionInfo (Uid, FishId, MaxSize, MinSize, CreateTime) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(uid)
            .bind(fish_id)
            .bind(size)
            .bind(size)
            .bind(now)
            .execute(pool)
            .await?;
            FishingCollectionInfo {
                uid,
                fish_id,
                max_size: size,
                min_size: size,
                create_time: Some(now),
            }
        }
    };
    Ok(row)
}
