use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::evil::evil_castle_rogue_like_give_up_info::EvilCastleRogueLikeGiveUpInfo;
/// Insert a full JSON array of EvilCastleRogueLikeGiveUpInfo records for a UID.
pub async fn insert_evil_castle_rogue_like_give_up_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("evilCastleRogueLikeGiveUpInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_evil_castle_rogue_like_give_up_info: missing or invalid 'evilCastleRogueLikeGiveUpInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let obsidian = entry
            .get("obsidian")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO EvilCastleRogueLikeGiveUpInfo (
    Uid,
    Obsidian
) VALUES (
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&obsidian)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single EvilCastleRogueLikeGiveUpInfo record from a Rust struct.
pub async fn add_evil_castle_rogue_like_give_up_info(pool: &SqlitePool, data: &EvilCastleRogueLikeGiveUpInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EvilCastleRogueLikeGiveUpInfo (
    Uid,
    Obsidian
) VALUES (
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.obsidian)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_evil_castle_rogue_like_give_up_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<EvilCastleRogueLikeGiveUpInfo>> {
    sqlx::query_as::<_, EvilCastleRogueLikeGiveUpInfo>("SELECT * FROM EvilCastleRogueLikeGiveUpInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all EvilCastleRogueLikeGiveUpInfo rows for a UID.
pub async fn delete_evil_castle_rogue_like_give_up_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EvilCastleRogueLikeGiveUpInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}