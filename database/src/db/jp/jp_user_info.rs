use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::jp::jp_user_info::JpUserInfo;
/// Insert a full JSON array of JpUserInfo records for a UID.
pub async fn insert_jp_user_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("jpUserInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_jp_user_info: missing or invalid 'jpUserInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        let accumulated_payment_amount = entry
            .get("accumulatedPaymentAmount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let date_of_birth = entry
            .get("dateOfBirth")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;

        sqlx::query(
            r#"
INSERT INTO JpUserInfo (
    Uid,
    AccumulatedPaymentAmount,
    DateOfBirth
) VALUES (
    ?,
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&accumulated_payment_amount)
        .bind(&date_of_birth)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single JpUserInfo record from a Rust struct.
pub async fn add_jp_user_info(pool: &SqlitePool, data: &JpUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO JpUserInfo (
    Uid,
    AccumulatedPaymentAmount,
    DateOfBirth
) VALUES (
    ?,
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.accumulated_payment_amount)
    .bind(&data.date_of_birth)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn set_date_of_birth(pool: &SqlitePool, uid: i64, date_of_birth: i32) -> sqlx::Result<()> {
    let existing = get_jp_user_info(pool, uid).await?;
    if let Some(row) = existing.into_iter().next() {
        sqlx::query("UPDATE JpUserInfo SET DateOfBirth = ? WHERE \"Index\" = ?")
            .bind(date_of_birth)
            .bind(row.index)
            .execute(pool)
            .await?;
    } else {
        add_jp_user_info(
            pool,
            &crate::models::game::jp::jp_user_info::JpUserInfo { index: 0, uid, accumulated_payment_amount: Some(0.0), date_of_birth: Some(date_of_birth) },
        )
        .await?;
    }
    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_jp_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<JpUserInfo>> {
    sqlx::query_as::<_, JpUserInfo>("SELECT * FROM JpUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all JpUserInfo rows for a UID.
pub async fn delete_jp_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM JpUserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}