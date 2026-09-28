use crate::models::game::charge::charge_cost_info::ChargeCostInfo;
use sqlx::SqlitePool;

/// Add a single ChargeCostInfo record from a Rust struct.
pub async fn add_charge_cost_info(pool: &SqlitePool, data: &ChargeCostInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ChargeCostInfo (
    Uid,
    CostTimeInfoIndex,
    EventScheduleInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.cost_time_info_index)
    .bind(&data.event_schedule_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_charge_cost_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ChargeCostInfo>> {
    sqlx::query_as::<_, ChargeCostInfo>("SELECT * FROM ChargeCostInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ChargeCostInfo rows for a UID.
pub async fn delete_charge_cost_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ChargeCostInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
