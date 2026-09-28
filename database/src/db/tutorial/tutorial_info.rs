use crate::models::game::tutorial::tutorial_info::TutorialInfo;
use serde_json::Value;
use sqlx::SqlitePool;

/// Insert tutorial clear IDs for a user from the given JSON object.
pub async fn insert_tutorial_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    // Expect data["tutorialClearId"] as array
    let tutorial_clear_id_array = data.get("tutorialClearId").and_then(|v| v.as_array());
    if tutorial_clear_id_array.is_none() {
        eprintln!("insert_tutorial_info: missing 'tutorialClearId' array");
        return Ok(());
    }

    for item in tutorial_clear_id_array.unwrap() {
        let tutorial_clear_id = item.as_i64().unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO TutorialInfo (
    Uid,
    TutorialClearId
) VALUES (?, ?)
"#,
        )
        .bind(uid)
        .bind(&tutorial_clear_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single TutorialInfo record from a Rust struct.
pub async fn add_tutorial_info(pool: &SqlitePool, data: &TutorialInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TutorialInfo (
    Uid,
    TutorialClearId
) VALUES (?, ?)
"#,
    )
    .bind(&data.uid)
    .bind(&data.tutorial_clear_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all TutorialInfo rows for a user.
pub async fn get_tutorial_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<TutorialInfo>> {
    sqlx::query_as::<_, TutorialInfo>("SELECT * FROM TutorialInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all TutorialInfo rows for a user.
pub async fn delete_tutorial_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TutorialInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
