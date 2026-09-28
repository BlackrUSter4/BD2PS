use crate::models::game::monster::monster_hunt_rank_info::MonsterHuntRankInfo;
use serde_json::Value;
use sqlx::SqlitePool;
/// Insert a full JSON array of MonsterHuntRankInfo records for a UID.
pub async fn insert_monster_hunt_rank_info(
    pool: &SqlitePool,
    data: &Value,
    uid: i64,
) -> sqlx::Result<()> {
    let arr = match data.get("monsterHuntRankInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!(
                "insert_monster_hunt_rank_info: missing or invalid 'monsterHuntRankInfo' array"
            );
            return Ok(());
        }
    };

    for entry in arr {
        // Handle repeated nested MonsterHuntRankUserInfo - extract InvenIndex values
        let user_rank_info_index =
            if let Some(nested_arr) = entry.get("userRankInfo").and_then(|v| v.as_array()) {
                let index: Vec<i64> = nested_arr
                    .iter()
                    .filter_map(|item| item.get("invenIndex").and_then(|v| v.as_i64()))
                    .collect();

                if index.is_empty() {
                    None
                } else {
                    Some(serde_json::to_string(&index).unwrap())
                }
            } else {
                None
            };
        // Handle single nested MonsterHuntRankUserInfo - extract InvenIndex
        let my_rank_info_index = entry
            .get("myRankInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO MonsterHuntRankInfo (
    Uid,
    UserRankInfoIndex,
    MyRankInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
        )
        .bind(uid)
        .bind(&user_rank_info_index)
        .bind(&my_rank_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single MonsterHuntRankInfo record from a Rust struct.
pub async fn add_monster_hunt_rank_info(
    pool: &SqlitePool,
    data: &MonsterHuntRankInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MonsterHuntRankInfo (
    Uid,
    UserRankInfoIndex,
    MyRankInfoIndex
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.user_rank_info_index)
    .bind(&data.my_rank_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_monster_hunt_rank_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<MonsterHuntRankInfo>> {
    sqlx::query_as::<_, MonsterHuntRankInfo>("SELECT * FROM MonsterHuntRankInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all MonsterHuntRankInfo rows for a UID.
pub async fn delete_monster_hunt_rank_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MonsterHuntRankInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
