use crate::models::game::field_event_spawn::field_event_spawn_daily_count::FieldEventSpawnDailyCount;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64, date: &str) -> FieldEventSpawnDailyCount {
    sqlx::query_as::<_, FieldEventSpawnDailyCount>(
        "SELECT * FROM FieldEventSpawnDailyCount WHERE Uid = ? AND Date = ?",
    )
    .bind(uid)
    .bind(date)
    .fetch_optional(pool)
    .await
    .ok()
    .flatten()
    .unwrap_or(FieldEventSpawnDailyCount {
        uid,
        date: date.to_string(),
        normal_count: 0,
        special_count: 0,
    })
}

pub async fn increment(pool: &SqlitePool, uid: i64, date: &str, is_special: bool) -> sqlx::Result<()> {
    if is_special {
        sqlx::query(
            r#"
INSERT INTO FieldEventSpawnDailyCount (Uid, Date, NormalCount, SpecialCount) VALUES (?, ?, 0, 1)
ON CONFLICT(Uid, Date) DO UPDATE SET SpecialCount = SpecialCount + 1
"#,
        )
    } else {
        sqlx::query(
            r#"
INSERT INTO FieldEventSpawnDailyCount (Uid, Date, NormalCount, SpecialCount) VALUES (?, ?, 1, 0)
ON CONFLICT(Uid, Date) DO UPDATE SET NormalCount = NormalCount + 1
"#,
        )
    }
    .bind(uid)
    .bind(date)
    .execute(pool)
    .await?;
    Ok(())
}
