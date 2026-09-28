use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<i32>> {
    let rows: Vec<(i32,)> = sqlx::query_as("SELECT SkinId FROM FishingBoatSkinOwned WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

pub async fn is_owned(pool: &SqlitePool, uid: i64, skin_id: i32) -> sqlx::Result<bool> {
    let row: Option<(i32,)> =
        sqlx::query_as("SELECT SkinId FROM FishingBoatSkinOwned WHERE Uid = ? AND SkinId = ?")
            .bind(uid)
            .bind(skin_id)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

pub async fn add(pool: &SqlitePool, uid: i64, skin_id: i32) -> sqlx::Result<()> {
    sqlx::query("INSERT OR IGNORE INTO FishingBoatSkinOwned (Uid, SkinId) VALUES (?, ?)")
        .bind(uid)
        .bind(skin_id)
        .execute(pool)
        .await?;
    Ok(())
}
