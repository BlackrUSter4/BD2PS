use crate::models::game::char::char_auto_revive_setting::CharAutoReviveSetting;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<CharAutoReviveSetting>> {
    sqlx::query_as::<_, CharAutoReviveSetting>(
        "SELECT * FROM CharAutoReviveSetting WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// Upsert the account's auto-revive setting.
pub async fn upsert(
    pool: &SqlitePool,
    uid: i64,
    can_auto_revive: bool,
    casting_char_inven_index: Option<i64>,
) -> sqlx::Result<()> {
    if get(pool, uid).await?.is_some() {
        sqlx::query(
            "UPDATE CharAutoReviveSetting SET CanAutoRevive = ?, CastingCharInvenIndex = ? WHERE Uid = ?",
        )
        .bind(can_auto_revive)
        .bind(casting_char_inven_index)
        .bind(uid)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            "INSERT INTO CharAutoReviveSetting (Uid, CanAutoRevive, CastingCharInvenIndex) VALUES (?, ?, ?)",
        )
        .bind(uid)
        .bind(can_auto_revive)
        .bind(casting_char_inven_index)
        .execute(pool)
        .await?;
    }
    Ok(())
}
