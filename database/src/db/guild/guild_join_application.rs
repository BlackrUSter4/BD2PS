use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct GuildJoinApplication {
    #[sqlx(rename = "Index")]
    pub index: i64,
    #[sqlx(rename = "GuildId")]
    pub guild_id: i64,
    #[sqlx(rename = "ApplicantUid")]
    pub applicant_uid: i64,
    #[sqlx(rename = "ApplicantUserId")]
    pub applicant_user_id: Option<String>,
    #[sqlx(rename = "Date")]
    pub date: Option<i64>,
}

pub async fn insert(pool: &SqlitePool, guild_id: i64, applicant_uid: i64, applicant_user_id: &str, date: i64) -> sqlx::Result<()> {
    sqlx::query("INSERT INTO GuildJoinApplication (GuildId, ApplicantUid, ApplicantUserId, Date) VALUES (?, ?, ?, ?)")
        .bind(guild_id)
        .bind(applicant_uid)
        .bind(applicant_user_id)
        .bind(date)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_for_guild(pool: &SqlitePool, guild_id: i64) -> sqlx::Result<Vec<GuildJoinApplication>> {
    sqlx::query_as::<_, GuildJoinApplication>("SELECT * FROM GuildJoinApplication WHERE GuildId = ?")
        .bind(guild_id)
        .fetch_all(pool)
        .await
}

pub async fn find(pool: &SqlitePool, guild_id: i64, applicant_uid: i64) -> sqlx::Result<Option<GuildJoinApplication>> {
    sqlx::query_as::<_, GuildJoinApplication>("SELECT * FROM GuildJoinApplication WHERE GuildId = ? AND ApplicantUid = ?")
        .bind(guild_id)
        .bind(applicant_uid)
        .fetch_optional(pool)
        .await
}

pub async fn list_for_applicant(pool: &SqlitePool, applicant_uid: i64) -> sqlx::Result<Vec<GuildJoinApplication>> {
    sqlx::query_as::<_, GuildJoinApplication>("SELECT * FROM GuildJoinApplication WHERE ApplicantUid = ?")
        .bind(applicant_uid)
        .fetch_all(pool)
        .await
}

pub async fn delete(pool: &SqlitePool, guild_id: i64, applicant_uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildJoinApplication WHERE GuildId = ? AND ApplicantUid = ?")
        .bind(guild_id)
        .bind(applicant_uid)
        .execute(pool)
        .await?;
    Ok(())
}
