use crate::models::game::total::total_battle_power::TotalBattlePower;
use sqlx::SqlitePool;

/// Add a single TotalBattlePower record from a Rust struct.
pub async fn add_total_battle_power(
    pool: &SqlitePool,
    data: &TotalBattlePower,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TotalBattlePower (
    Uid,
    TotalBattlePower
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.total_battle_power)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_total_battle_power(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<TotalBattlePower>> {
    sqlx::query_as::<_, TotalBattlePower>("SELECT * FROM TotalBattlePower WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Record a newly-reported total battle power if it's a new personal best, returning the
/// account's real highest-ever recorded value.
pub async fn upsert_highest(pool: &SqlitePool, uid: i64, reported: i32) -> sqlx::Result<i32> {
    let existing = get_total_battle_power(pool, uid).await?.into_iter().next();
    match existing {
        Some(row) => {
            let highest = row.total_battle_power.unwrap_or(0).max(reported);
            if Some(highest) != row.total_battle_power {
                sqlx::query("UPDATE TotalBattlePower SET TotalBattlePower = ? WHERE \"Index\" = ?")
                    .bind(highest)
                    .bind(row.index)
                    .execute(pool)
                    .await?;
            }
            Ok(highest)
        }
        None => {
            add_total_battle_power(pool, &TotalBattlePower { index: 0, uid, total_battle_power: Some(reported) }).await?;
            Ok(reported)
        }
    }
}

/// Delete all TotalBattlePower rows for a UID.
pub async fn delete_total_battle_power(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM TotalBattlePower WHERE Uid = ?")
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
) -> sqlx::Result<TotalBattlePower> {
    sqlx::query_as::<_, TotalBattlePower>(
        "SELECT * FROM TotalBattlePower WHERE Uid = ? AND Index = ?",
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
) -> sqlx::Result<Vec<TotalBattlePower>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM TotalBattlePower WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, TotalBattlePower>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &TotalBattlePower) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO TotalBattlePower (
    Uid,
    TotalBattlePower
) VALUES (
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.total_battle_power)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
