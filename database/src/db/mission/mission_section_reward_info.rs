use crate::models::game::mission::mission_section_reward_info::MissionSectionRewardInfo;
use sqlx::SqlitePool;

/// Add a single MissionSectionRewardInfo record from a Rust struct.
pub async fn add_mission_section_reward_info(
    pool: &SqlitePool,
    data: &MissionSectionRewardInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MissionSectionRewardInfo (
    Uid,
    GroupType,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_type)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mission_section_reward_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MissionSectionRewardInfo>> {
    sqlx::query_as::<_, MissionSectionRewardInfo>(
        "SELECT * FROM MissionSectionRewardInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

pub async fn is_claimed(pool: &SqlitePool, uid: i64, group_type: i32, id: i32) -> sqlx::Result<bool> {
    let row = sqlx::query_as::<_, MissionSectionRewardInfo>(
        "SELECT * FROM MissionSectionRewardInfo WHERE Uid = ? AND GroupType = ? AND Id = ?",
    )
    .bind(uid)
    .bind(group_type)
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// Delete all MissionSectionRewardInfo rows for a UID.
pub async fn delete_mission_section_reward_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MissionSectionRewardInfo WHERE Uid = ?")
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
) -> sqlx::Result<MissionSectionRewardInfo> {
    sqlx::query_as::<_, MissionSectionRewardInfo>(
        "SELECT * FROM MissionSectionRewardInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MissionSectionRewardInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MissionSectionRewardInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MissionSectionRewardInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MissionSectionRewardInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MissionSectionRewardInfo (
    Uid,
    GroupType,
    Id
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.group_type)
    .bind(&data.id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
