use crate::models::game::mail::mail_info::MailInfo;
use sqlx::SqlitePool;

/// Add a single MailInfo record from a Rust struct.
pub async fn add_mail_info(pool: &SqlitePool, data: &MailInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO MailInfo (
    Uid,
    InvenIndex,
    Type,
    MailId,
    SenderText,
    TitleText,
    MessageText,
    RewardExpireTime,
    IsOpen,
    OpenTime,
    CreateTime,
    HistoryDeleteTime,
    IsCash
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.inven_index)
    .bind(&data.r#type)
    .bind(&data.mail_id)
    .bind(&data.sender_text)
    .bind(&data.title_text)
    .bind(&data.message_text)
    .bind(&data.reward_expire_time)
    .bind(&data.is_open)
    .bind(&data.open_time)
    .bind(&data.create_time)
    .bind(&data.history_delete_time)
    .bind(&data.is_cash)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_mail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<MailInfo>> {
    sqlx::query_as::<_, MailInfo>("SELECT * FROM MailInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_inven_index(pool: &SqlitePool, uid: i64, inven_index: i64) -> sqlx::Result<Vec<MailInfo>> {
    sqlx::query_as::<_, MailInfo>("SELECT * FROM MailInfo WHERE Uid = ? AND InvenIndex = ?")
        .bind(uid)
        .bind(inven_index)
        .fetch_all(pool)
        .await
}

pub async fn mark_opened(pool: &SqlitePool, uid: i64, inven_index: i64, open_time: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE MailInfo SET IsOpen = 1, OpenTime = ? WHERE Uid = ? AND InvenIndex = ?")
        .bind(open_time)
        .bind(uid)
        .bind(inven_index)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all MailInfo rows for a UID.
pub async fn delete_mail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM MailInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<MailInfo> {
    sqlx::query_as::<_, MailInfo>("SELECT * FROM MailInfo WHERE Uid = ? AND Index = ?")
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
) -> sqlx::Result<Vec<MailInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM MailInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, MailInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &MailInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO MailInfo (
    Uid,
    InvenIndex,
    Type,
    MailId,
    SenderText,
    TitleText,
    MessageText,
    RewardExpireTime,
    IsOpen,
    OpenTime,
    CreateTime,
    HistoryDeleteTime,
    IsCash
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
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
    .bind(&data.inven_index)
    .bind(&data.r#type)
    .bind(&data.mail_id)
    .bind(&data.sender_text)
    .bind(&data.title_text)
    .bind(&data.message_text)
    .bind(&data.reward_expire_time)
    .bind(&data.is_open)
    .bind(&data.open_time)
    .bind(&data.create_time)
    .bind(&data.history_delete_time)
    .bind(&data.is_cash)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
