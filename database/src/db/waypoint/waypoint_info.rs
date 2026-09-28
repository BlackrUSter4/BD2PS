use crate::models::game::waypoint::waypoint_info::WaypointInfo;
use sqlx::SqlitePool;

/// Add a single WaypointInfo record from a Rust struct.
pub async fn add_waypoint_info(pool: &SqlitePool, data: &WaypointInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO WaypointInfo (
    Uid,
    WaypointId
) VALUES (?, ?)
"#,
    )
    .bind(&data.uid)
    .bind(&data.waypoint_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_waypoint_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<WaypointInfo>> {
    sqlx::query_as::<_, WaypointInfo>("SELECT * FROM WaypointInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all WaypointInfo rows for a UID.
pub async fn delete_waypoint_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM WaypointInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
