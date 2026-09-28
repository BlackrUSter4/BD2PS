use crate::models::game::event::event_hub_setting_info::EventHubSettingInfo;
use sqlx::SqlitePool;

/// Add a single EventHubSettingInfo record from a Rust struct.
pub async fn add_event_hub_setting_info(
    pool: &SqlitePool,
    data: &EventHubSettingInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO EventHubSettingInfo (
    Uid,
    HubInfoIndex,
    Slot,
    HubContentType,
    EventUid
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.hub_info_index)
    .bind(data.slot)
    .bind(data.hub_content_type)
    .bind(data.event_uid)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_event_hub_setting_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<EventHubSettingInfo>> {
    sqlx::query_as::<_, EventHubSettingInfo>("SELECT * FROM EventHubSettingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Fetch all setting rows belonging to a specific EventHubInfo row.
pub async fn get_by_hub_info_index(
    pool: &SqlitePool,
    uid: i64,
    hub_info_index: i64,
) -> sqlx::Result<Vec<EventHubSettingInfo>> {
    sqlx::query_as::<_, EventHubSettingInfo>(
        "SELECT * FROM EventHubSettingInfo WHERE Uid = ? AND HubInfoIndex = ?",
    )
    .bind(uid)
    .bind(hub_info_index)
    .fetch_all(pool)
    .await
}

/// Delete all EventHubSettingInfo rows for a UID.
pub async fn delete_event_hub_setting_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM EventHubSettingInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: i64,
) -> sqlx::Result<EventHubSettingInfo> {
    sqlx::query_as::<_, EventHubSettingInfo>(
        "SELECT * FROM EventHubSettingInfo WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &EventHubSettingInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO EventHubSettingInfo (
    Uid,
    HubInfoIndex,
    Slot,
    HubContentType,
    EventUid
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.hub_info_index)
    .bind(data.slot)
    .bind(data.hub_content_type)
    .bind(data.event_uid)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
