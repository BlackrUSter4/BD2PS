use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::recipe::recipe_info::RecipeInfo;
/// Insert a full JSON array of RecipeInfo records for a UID.
pub async fn insert_recipe_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("recipeInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_recipe_info: missing or invalid 'recipeInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let recipe_id_array = entry.get("recipeId").and_then(|v| v.as_array());
        if let Some(values) = recipe_id_array {
            for item in values {
                let seq = entry
                    .get("seq")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default() as i32;
                let recipe_id = item.as_i64().unwrap_or_default() as i32;

                sqlx::query(
                    r#"
INSERT INTO RecipeInfo (
    Uid,
    Seq,
    RecipeId
) VALUES (
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(&seq)
                .bind(&recipe_id)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single RecipeInfo record from a Rust struct.
pub async fn add_recipe_info(pool: &SqlitePool, data: &RecipeInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO RecipeInfo (
    Uid,
    Seq,
    RecipeId
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.seq)
    .bind(&data.recipe_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_recipe_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<RecipeInfo>> {
    sqlx::query_as::<_, RecipeInfo>("SELECT * FROM RecipeInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all RecipeInfo rows for a UID.
pub async fn is_unlocked(pool: &SqlitePool, uid: i64, recipe_id: i32) -> sqlx::Result<bool> {
    let row = sqlx::query_as::<_, RecipeInfo>("SELECT * FROM RecipeInfo WHERE Uid = ? AND RecipeId = ?")
        .bind(uid)
        .bind(recipe_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn delete_recipe_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM RecipeInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}