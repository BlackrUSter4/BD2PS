use crate::models::game::fishing::fishing_bite_session::FishingBiteSession;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<FishingBiteSession>> {
    sqlx::query_as::<_, FishingBiteSession>("SELECT * FROM FishingBiteSession WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn upsert(pool: &SqlitePool, data: &FishingBiteSession) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FishingBiteSession (Uid, FishId, Size, Hp, Stamina, StartTime)
VALUES (?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    FishId = excluded.FishId, Size = excluded.Size, Hp = excluded.Hp,
    Stamina = excluded.Stamina, StartTime = excluded.StartTime
"#,
    )
    .bind(data.uid)
    .bind(data.fish_id)
    .bind(data.size)
    .bind(data.hp)
    .bind(data.stamina)
    .bind(data.start_time)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn set_hp(pool: &SqlitePool, uid: i64, hp: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE FishingBiteSession SET Hp = ? WHERE Uid = ?")
        .bind(hp)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_stamina(pool: &SqlitePool, uid: i64, stamina: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE FishingBiteSession SET Stamina = ? WHERE Uid = ?")
        .bind(stamina)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn clear(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM FishingBiteSession WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
