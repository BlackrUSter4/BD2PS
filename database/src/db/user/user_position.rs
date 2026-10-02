use crate::models::game::user::user_position::UserPosition;
use sqlx::SqlitePool;

/// Resolve the account's CURRENT pack id for server logic that needs one but
/// has no pack_id in its own request proto (unlike e.g. FieldObjectResearchRequest,
/// which the client already tells us directly — see CLIENT_UPDATE.md's 2026-10-02
/// "(packId, id) composite key" entries).
///
/// `UserPosition.PackId` itself is never populated by any real handler (only
/// `SaveUserPositionRequest` writes this table, and it only ever sets
/// `PackPosition`, a JSON blob like `{"MapId":1,"PlayerPosition":{...}}` —
/// `PackId` stays NULL forever). The real, live-updated source of the
/// account's current pack is that blob's `MapId`, resolved through
/// `MapTable.packId` (every map belongs to exactly one pack, and MapTable's
/// `id` is a real global id, unlike the small per-pack-local ids this fix's
/// sibling tables needed a composite key for). Falls back to pack 1 (the
/// game's starting pack) when no position is saved yet or the saved MapId
/// doesn't resolve — same convention `pack_in_game_info.rs` already uses for
/// the position itself.
pub async fn get_current_pack_id(pool: &SqlitePool, uid: i64) -> i32 {
    let position: Option<String> =
        sqlx::query_scalar("SELECT PackPosition FROM UserPosition WHERE Uid = ?")
            .bind(uid)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();

    position
        .and_then(|pos| serde_json::from_str::<serde_json::Value>(&pos).ok())
        .and_then(|v| v.get("MapId").and_then(|m| m.as_i64()))
        .and_then(|map_id| data::exceldb::get().maptable.get(map_id as i32).map(|m| m.pack_id))
        .unwrap_or(1)
}

/// Add a single UserPosition record from a Rust struct.
pub async fn add_user_position(pool: &SqlitePool, data: &UserPosition) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO UserPosition (
    Uid,
    PackId,
    PackPosition
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.pack_position)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_user_position(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<UserPosition>> {
    sqlx::query_as::<_, UserPosition>("SELECT * FROM UserPosition WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all UserPosition rows for a UID.
pub async fn delete_user_position(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM UserPosition WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<UserPosition> {
    sqlx::query_as::<_, UserPosition>("SELECT * FROM UserPosition WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<UserPosition>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM UserPosition WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, UserPosition>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &UserPosition) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO UserPosition (
    Uid,
    PackId,
    PackPosition
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.pack_id)
    .bind(&data.pack_position)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
