use crate::models::game::age_gate::age_gate_info::AgeGateInfo;
use sqlx::SqlitePool;

pub async fn save(pool: &SqlitePool, info: &AgeGateInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AgeGateInfo (Uid, IsJp, Year, Month, Day) VALUES (?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET IsJp = excluded.IsJp, Year = excluded.Year, Month = excluded.Month, Day = excluded.Day
"#,
    )
    .bind(info.uid)
    .bind(info.is_jp)
    .bind(info.year)
    .bind(info.month)
    .bind(info.day)
    .execute(pool)
    .await?;
    Ok(())
}
