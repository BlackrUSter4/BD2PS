use crate::models::game::deck::deck_costume_setting_info::DeckCostumeSettingInfo;
use sqlx::SqlitePool;

/// Add a single DeckCostumeSettingInfo record from a Rust struct.
pub async fn add_deck_costume_setting_info(
    pool: &SqlitePool,
    data: &DeckCostumeSettingInfo,
) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO DeckCostumeSettingInfo (
    Uid,
    CharInvenIndex,
    CostumeInvenIndexSeq
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.char_inven_index)
    .bind(data.costume_inven_index_seq)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_deck_costume_setting_info(
    pool: &SqlitePool,
    uid: i64,
) -> sqlx::Result<Vec<DeckCostumeSettingInfo>> {
    sqlx::query_as::<_, DeckCostumeSettingInfo>(
        "SELECT * FROM DeckCostumeSettingInfo WHERE Uid = ?",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// Replace the saved costume sequence for one character (delete + reinsert).
pub async fn replace_for_char(
    pool: &SqlitePool,
    uid: i64,
    char_inven_index: i64,
    costume_inven_index_seq: &[i64],
) -> sqlx::Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM DeckCostumeSettingInfo WHERE Uid = ? AND CharInvenIndex = ?")
        .bind(uid)
        .bind(char_inven_index)
        .execute(&mut *tx)
        .await?;
    for seq in costume_inven_index_seq {
        sqlx::query(
            "INSERT INTO DeckCostumeSettingInfo (Uid, CharInvenIndex, CostumeInvenIndexSeq) VALUES (?, ?, ?)",
        )
        .bind(uid)
        .bind(char_inven_index)
        .bind(seq)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}

/// Delete all DeckCostumeSettingInfo rows for a UID.
pub async fn delete_deck_costume_setting_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM DeckCostumeSettingInfo WHERE Uid = ?")
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
) -> sqlx::Result<DeckCostumeSettingInfo> {
    sqlx::query_as::<_, DeckCostumeSettingInfo>(
        "SELECT * FROM DeckCostumeSettingInfo WHERE Uid = ? AND \"Index\" = ?",
    )
    .bind(uid)
    .bind(index)
    .fetch_one(pool)
    .await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &DeckCostumeSettingInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO DeckCostumeSettingInfo (
    Uid,
    CharInvenIndex,
    CostumeInvenIndexSeq
) VALUES (
    ?,
    ?,
    ?
)
"#,
    )
    .bind(data.uid)
    .bind(data.char_inven_index)
    .bind(data.costume_inven_index_seq)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
