use crate::models::game::tactics_bingo::tactics_bingo_deck_info::TacticsBingoDeckInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> Option<TacticsBingoDeckInfo> {
    sqlx::query_as::<_, TacticsBingoDeckInfo>("SELECT * FROM TacticsBingoDeckInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

pub async fn save(pool: &SqlitePool, uid: i64, deck_info: Option<&str>) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO TacticsBingoDeckInfo (Uid, DeckInfo) VALUES (?, ?)
ON CONFLICT(Uid) DO UPDATE SET DeckInfo = excluded.DeckInfo
"#,
    )
    .bind(uid)
    .bind(deck_info)
    .execute(pool)
    .await?;
    Ok(())
}
