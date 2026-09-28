use crate::models::game::hunting::hunting_ground_info::HuntingGroundInfo;
use sqlx::SqlitePool;

/// Add a single HuntingGroundInfo record from a Rust struct.
pub async fn add_hunting_ground_info(
    pool: &SqlitePool,
    data: &HuntingGroundInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO HuntingGroundInfo (
    Uid,
    IsAuto,
    CurrentId,
    HighestId,
    PackId
) VALUES (
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
    .bind(&data.is_auto)
    .bind(&data.current_id)
    .bind(&data.highest_id)
    .bind(&data.pack_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_hunting_ground_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<HuntingGroundInfo>> {
    sqlx::query_as::<_, HuntingGroundInfo>("SELECT * FROM HuntingGroundInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all HuntingGroundInfo rows for a UID.
pub async fn delete_hunting_ground_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM HuntingGroundInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn link_monster_to_hunting_ground(
    pool: &SqlitePool,
    hunting_ground_index: i64,
    monster_index: i64,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO HuntingGroundMonster (HuntingGroundIndex, MonsterIndex)
VALUES (?, ?)
"#,
    )
    .bind(hunting_ground_index)
    .bind(monster_index)
    .execute(pool)
    .await?;
    Ok(())
}
