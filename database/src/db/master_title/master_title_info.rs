use crate::models::game::master_title::master_title_info::MasterTitleInfo;
use sqlx::SqlitePool;

pub async fn get_or_default(pool: &SqlitePool, uid: i64) -> MasterTitleInfo {
    sqlx::query_as::<_, MasterTitleInfo>("SELECT * FROM MasterTitleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(MasterTitleInfo { uid, name: None, month: None, day: None })
}

pub async fn save(pool: &SqlitePool, info: &MasterTitleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MasterTitleInfo (Uid, Name, Month, Day) VALUES (?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET Name = excluded.Name, Month = excluded.Month, Day = excluded.Day
"#,
    )
    .bind(info.uid)
    .bind(&info.name)
    .bind(info.month)
    .bind(info.day)
    .execute(pool)
    .await?;
    Ok(())
}
