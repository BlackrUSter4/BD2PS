use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::notice::notice_detail_info::NoticeDetailInfo;
/// Insert a full JSON array of NoticeDetailInfo records for a UID.
pub async fn insert_notice_detail_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("noticeDetailInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_notice_detail_info: missing or invalid 'noticeDetailInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle single nested NoticeInfo - extract InvenIndex
        let notice_info_index = entry
            .get("noticeInfo")
            .and_then(|item| item.get("invenIndex"))
            .and_then(|v| v.as_i64());

        sqlx::query(
            r#"
INSERT INTO NoticeDetailInfo (
    Uid,
    NoticeInfoIndex
) VALUES (
    ?,
    ?
)
"#
        )
        .bind(uid)
        .bind(&notice_info_index)
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Add a single NoticeDetailInfo record from a Rust struct.
pub async fn add_notice_detail_info(pool: &SqlitePool, data: &NoticeDetailInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO NoticeDetailInfo (
    Uid,
    NoticeInfoIndex
) VALUES (
    ?,
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.notice_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_notice_detail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<NoticeDetailInfo>> {
    sqlx::query_as::<_, NoticeDetailInfo>("SELECT * FROM NoticeDetailInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all NoticeDetailInfo rows for a UID.
pub async fn delete_notice_detail_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM NoticeDetailInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}