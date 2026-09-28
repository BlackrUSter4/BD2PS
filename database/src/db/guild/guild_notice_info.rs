use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::guild::guild_notice_info::GuildNoticeInfo;
/// Insert a full JSON array of GuildNoticeInfo records for a UID.
pub async fn insert_guild_notice_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("guildNoticeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_guild_notice_info: missing or invalid 'guildNoticeInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let notice = entry
            .get("notice")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let date = entry
            .get("date")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        sqlx::query(
            r#"
INSERT INTO GuildNoticeInfo (
    Uid,
    Notice,
    Date
) VALUES (
    ?,
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&notice)
        .bind(&date)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single GuildNoticeInfo record from a Rust struct.
pub async fn add_guild_notice_info(pool: &SqlitePool, data: &GuildNoticeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO GuildNoticeInfo (
    Uid,
    Notice,
    Date
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.notice)
    .bind(&data.date)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_guild_notice_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<GuildNoticeInfo>> {
    sqlx::query_as::<_, GuildNoticeInfo>("SELECT * FROM GuildNoticeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all GuildNoticeInfo rows for a UID.
pub async fn delete_guild_notice_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM GuildNoticeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}