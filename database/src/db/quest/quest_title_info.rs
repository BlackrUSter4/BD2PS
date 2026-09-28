use crate::models::game::quest::quest_title_info::QuestTitleInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of QuestTitleInfo records for a UID.
pub async fn insert_quest_title_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("questTitleInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_quest_title_info: missing or invalid 'questTitleInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let title_id = entry
            .get("titleId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let max_clear_id = entry
            .get("maxClearId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO QuestTitleInfo (
    Uid,
    TitleId,
    MaxClearId
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&title_id)
        .bind(&max_clear_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single QuestTitleInfo record from a Rust struct.
pub async fn add_quest_title_info(pool: &SqlitePool, data: &QuestTitleInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO QuestTitleInfo (
    Uid,
    TitleId,
    MaxClearId
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.title_id)
    .bind(&data.max_clear_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_quest_title_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<QuestTitleInfo>> {
    sqlx::query_as::<_, QuestTitleInfo>("SELECT * FROM QuestTitleInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all QuestTitleInfo rows for a UID.
pub async fn delete_quest_title_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM QuestTitleInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
