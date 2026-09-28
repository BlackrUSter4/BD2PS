use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::char::char_scout_info::CharScoutInfo;
/// Insert a full JSON array of CharScoutInfo records for a UID.
pub async fn insert_char_scout_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("charScoutInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_char_scout_info: missing or invalid 'charScoutInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle parallel repeated arrays - iterate all together
        let use_reset_count = entry
            .get("useResetCount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let next_auto_reset_time = entry
            .get("nextAutoResetTime")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();

        // Get all parallel arrays
        let appear_char_id_array = entry.get("appearCharId").and_then(|v| v.as_array());
        let scout_complete_char_id_array = entry.get("scoutCompleteCharId").and_then(|v| v.as_array());

        // Determine max length
        let len = 0
            .max(appear_char_id_array.map(|a| a.len()).unwrap_or(0))
            .max(scout_complete_char_id_array.map(|a| a.len()).unwrap_or(0));

        if len > 0 {
            // Insert one row per index (parallel iteration)
            for i in 0..len {
                let appear_char_id = appear_char_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let scout_complete_char_id = scout_complete_char_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                sqlx::query(
                    r#"
INSERT INTO CharScoutInfo (
    Uid,
    AppearCharId,
    UseResetCount,
    NextAutoResetTime,
    ScoutCompleteCharId
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
                )
                .bind(uid)
                .bind(appear_char_id)
                .bind(&use_reset_count)
                .bind(&next_auto_reset_time)
                .bind(scout_complete_char_id)
                .execute(pool)
                .await?;
            }
        } else {
            // No items, insert one row with NULLs for repeated fields
            sqlx::query(
                r#"
INSERT INTO CharScoutInfo (
    Uid,
    AppearCharId,
    UseResetCount,
    NextAutoResetTime,
    ScoutCompleteCharId
) VALUES (
    ?,
    NULL,
    ?,
    ?,
    NULL
)
"#
            )
            .bind(uid)
            .bind(&use_reset_count)
            .bind(&next_auto_reset_time)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Add a single CharScoutInfo record from a Rust struct.
pub async fn add_char_scout_info(pool: &SqlitePool, data: &CharScoutInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO CharScoutInfo (
    Uid,
    AppearCharId,
    UseResetCount,
    NextAutoResetTime,
    ScoutCompleteCharId
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.appear_char_id)
    .bind(&data.use_reset_count)
    .bind(&data.next_auto_reset_time)
    .bind(&data.scout_complete_char_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_char_scout_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<CharScoutInfo>> {
    sqlx::query_as::<_, CharScoutInfo>("SELECT * FROM CharScoutInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all CharScoutInfo rows for a UID.
pub async fn delete_char_scout_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM CharScoutInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Replace the whole featured lineup for a UID (used by initial-load and reset).
pub async fn replace_lineup(
    pool: &SqlitePool,
    uid: i64,
    appear_char_ids: &[i32],
    use_reset_count: i32,
    next_auto_reset_time: i64,
) -> sqlx::Result<()> {
    delete_char_scout_info(pool, uid).await?;
    for &appear_char_id in appear_char_ids {
        sqlx::query(
            r#"
INSERT INTO CharScoutInfo (
    Uid, AppearCharId, UseResetCount, NextAutoResetTime, ScoutCompleteCharId
) VALUES (?, ?, ?, ?, 0)
"#,
        )
        .bind(uid)
        .bind(appear_char_id)
        .bind(use_reset_count)
        .bind(next_auto_reset_time)
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Mark one featured slot as purchased (ScoutCompleteCharId = its own AppearCharId).
pub async fn mark_complete(pool: &SqlitePool, uid: i64, appear_char_id: i32) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE CharScoutInfo SET ScoutCompleteCharId = AppearCharId WHERE Uid = ? AND AppearCharId = ?",
    )
    .bind(uid)
    .bind(appear_char_id)
    .execute(pool)
    .await?;
    Ok(())
}