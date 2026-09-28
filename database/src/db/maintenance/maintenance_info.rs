use crate::models::game::maintenance::maintenance_info::MaintenanceInfo;
use sqlx::SqlitePool;

/// Add a single MaintenanceInfo record from a Rust struct.
pub async fn add_maintenance_info(pool: &SqlitePool, data: &MaintenanceInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MaintenanceInfo (
    Uid,
    MarketType,
    Version,
    BundleVersion,
    IsBundleUpdate,
    MaintenanceType,
    Date,
    RegionList,
    UseDsa,
    MaintenanceUrl,
    UseMaintenanceUrl,
    DownloadUrl,
    Notice,
    BundleVersionSd
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.market_type)
    .bind(&data.version)
    .bind(&data.bundle_version)
    .bind(&data.is_bundle_update)
    .bind(&data.maintenance_type)
    .bind(&data.date)
    .bind(&data.region_list)
    .bind(&data.use_dsa)
    .bind(&data.maintenance_url)
    .bind(&data.use_maintenance_url)
    .bind(&data.download_url)
    .bind(&data.notice)
    .bind(&data.bundle_version_sd)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_maintenance_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MaintenanceInfo>> {
    sqlx::query_as::<_, MaintenanceInfo>("SELECT * FROM MaintenanceInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MaintenanceInfo rows for a UID.
pub async fn delete_maintenance_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MaintenanceInfo WHERE Uid = ?")
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
) -> sqlx::Result<MaintenanceInfo> {
    sqlx::query_as::<_, MaintenanceInfo>(
        "SELECT * FROM MaintenanceInfo WHERE Uid = ? AND Index = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<MaintenanceInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MaintenanceInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MaintenanceInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MaintenanceInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MaintenanceInfo (
    Uid,
    MarketType,
    Version,
    BundleVersion,
    IsBundleUpdate,
    MaintenanceType,
    Date,
    RegionList,
    UseDsa,
    MaintenanceUrl,
    UseMaintenanceUrl,
    DownloadUrl,
    Notice,
    BundleVersionSd
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.market_type)
    .bind(&data.version)
    .bind(&data.bundle_version)
    .bind(&data.is_bundle_update)
    .bind(&data.maintenance_type)
    .bind(&data.date)
    .bind(&data.region_list)
    .bind(&data.use_dsa)
    .bind(&data.maintenance_url)
    .bind(&data.use_maintenance_url)
    .bind(&data.download_url)
    .bind(&data.notice)
    .bind(&data.bundle_version_sd)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
