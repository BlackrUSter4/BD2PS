use crate::models::game::npc::npc_reputation_info::NpcReputationInfo;
use sqlx::SqlitePool;

/// Add a single NpcReputationInfo record from a Rust struct.
pub async fn add_npc_reputation_info(
    pool: &SqlitePool,
    data: &NpcReputationInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO NpcReputationInfo (
    Uid,
    PackId,
    GroupId,
    NpcId,
    Point
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.group_id)
    .bind(&data.npc_id)
    .bind(&data.point)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_npc_reputation_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<NpcReputationInfo>> {
    sqlx::query_as::<_, NpcReputationInfo>("SELECT * FROM NpcReputationInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_npc(pool: &SqlitePool, uid: i64, npc_id: i32) -> sqlx::Result<Option<NpcReputationInfo>> {
    sqlx::query_as::<_, NpcReputationInfo>("SELECT * FROM NpcReputationInfo WHERE Uid = ? AND NpcId = ?")
        .bind(uid)
        .bind(npc_id)
        .fetch_optional(pool)
        .await
}

pub async fn add_point(pool: &SqlitePool, uid: i64, npc_id: i32, delta: i32) -> sqlx::Result<()> {
    if let Some(row) = get_by_npc(pool, uid, npc_id).await? {
        sqlx::query("UPDATE NpcReputationInfo SET Point = COALESCE(Point, 0) + ? WHERE \"Index\" = ?")
            .bind(delta)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_npc_reputation_info(
            pool,
            &NpcReputationInfo { index: 0, uid, pack_id: None, group_id: None, npc_id: Some(npc_id), point: Some(delta) },
        )
        .await?;
    }
    Ok(())
}

/// Delete all NpcReputationInfo rows for a UID.
pub async fn delete_npc_reputation_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM NpcReputationInfo WHERE Uid = ?")
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
) -> sqlx::Result<NpcReputationInfo> {
    sqlx::query_as::<_, NpcReputationInfo>(
        "SELECT * FROM NpcReputationInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<NpcReputationInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM NpcReputationInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, NpcReputationInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &NpcReputationInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO NpcReputationInfo (
    Uid,
    PackId,
    GroupId,
    NpcId,
    Point
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.group_id)
    .bind(&data.npc_id)
    .bind(&data.point)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
