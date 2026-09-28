use crate::models::game::attendance::attendance_info::AttendanceInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of AttendanceInfo records for a UID.
pub async fn insert_attendance_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("attendanceInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_attendance_info: missing or invalid 'attendanceInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated primitive fields - insert one row per value
        let attendance_time_array = entry.get("attendanceTime").and_then(|v| v.as_array());
        if let Some(values) = attendance_time_array {
            for item in values {
                let attendance_time = item.as_i64().unwrap_or_default();

                sqlx::query(
                    r#"
INSERT INTO AttendanceInfo (
    Uid,
    AttendanceTime
) VALUES (
    ?,
    ?
)
"#,
                )
                .bind(uid)
                .bind(&attendance_time)
                .execute(pool)
                .await?;
            }
        }
    }

    Ok(())
}

/// Add a single AttendanceInfo record from a Rust struct.
pub async fn add_attendance_info(pool: &SqlitePool, data: &AttendanceInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AttendanceInfo (
    Uid,
    AttendanceTime
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.attendance_time)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_attendance_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<AttendanceInfo>> {
    sqlx::query_as::<_, AttendanceInfo>("SELECT * FROM AttendanceInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all AttendanceInfo rows for a UID.
pub async fn delete_attendance_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM AttendanceInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
