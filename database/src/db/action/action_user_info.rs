use crate::models::game::action::action_user_info::ActionUserInfo;
use sqlx::SqlitePool;

/// Add a single ActionUserInfo record from a Rust struct.
pub async fn add_action_user_info(pool: &SqlitePool, data: &ActionUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ActionUserInfo (
    Uid,
    OwnerIndex,
    ActionCharId,
    ClientState,
    NetworkState,
    NetworkPing,
    CalcState
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.action_char_id)
    .bind(&data.client_state)
    .bind(&data.network_state)
    .bind(&data.network_ping)
    .bind(&data.calc_state)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_action_user_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<ActionUserInfo>> {
    sqlx::query_as::<_, ActionUserInfo>("SELECT * FROM ActionUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all ActionUserInfo rows for a UID.
pub async fn delete_action_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM ActionUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<ActionUserInfo> {
    sqlx::query_as::<_, ActionUserInfo>("SELECT * FROM ActionUserInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<ActionUserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM ActionUserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, ActionUserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &ActionUserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO ActionUserInfo (
    Uid,
    OwnerIndex,
    ActionCharId,
    ClientState,
    NetworkState,
    NetworkPing,
    CalcState
) VALUES (
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
    .bind(&data.owner_index)
    .bind(&data.action_char_id)
    .bind(&data.client_state)
    .bind(&data.network_state)
    .bind(&data.network_ping)
    .bind(&data.calc_state)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
