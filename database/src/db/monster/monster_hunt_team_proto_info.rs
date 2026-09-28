use crate::models::game::monster::monster_hunt_team_proto_info::MonsterHuntTeamProtoInfo;
use sqlx::SqlitePool;

/// Add a single MonsterHuntTeamProtoInfo record from a Rust struct.
pub async fn add_monster_hunt_team_proto_info(
    pool: &SqlitePool,
    data: &MonsterHuntTeamProtoInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntTeamProtoInfo (
    Uid,
    BlueCharProto,
    BlueEquipProto,
    BlueBuffProto,
    BlueCostumeProto
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
    .bind(&data.blue_char_proto)
    .bind(&data.blue_equip_proto)
    .bind(&data.blue_buff_proto)
    .bind(&data.blue_costume_proto)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_team_proto_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntTeamProtoInfo>> {
    sqlx::query_as::<_, MonsterHuntTeamProtoInfo>(
        "SELECT * FROM MonsterHuntTeamProtoInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Delete all MonsterHuntTeamProtoInfo rows for a UID.
pub async fn delete_monster_hunt_team_proto_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntTeamProtoInfo WHERE Uid = ?")
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
) -> sqlx::Result<MonsterHuntTeamProtoInfo> {
    sqlx::query_as::<_, MonsterHuntTeamProtoInfo>(
        "SELECT * FROM MonsterHuntTeamProtoInfo WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<MonsterHuntTeamProtoInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MonsterHuntTeamProtoInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MonsterHuntTeamProtoInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MonsterHuntTeamProtoInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MonsterHuntTeamProtoInfo (
    Uid,
    BlueCharProto,
    BlueEquipProto,
    BlueBuffProto,
    BlueCostumeProto
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
    .bind(&data.blue_char_proto)
    .bind(&data.blue_equip_proto)
    .bind(&data.blue_buff_proto)
    .bind(&data.blue_costume_proto)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
