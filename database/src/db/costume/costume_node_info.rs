use crate::models::game::costume::costume_node_info::CostumeNodeInfo;
use sqlx::SqlitePool;

pub async fn get_by_costume(
    pool: &SqlitePool,
    uid: i64,
    costume_inven_index: i64,
) -> sqlx::Result<Vec<CostumeNodeInfo>> {
    sqlx::query_as::<_, CostumeNodeInfo>(
        "SELECT * FROM CostumeNodeInfo WHERE Uid = ? AND CostumeInvenIndex = ?",
    )
    .bind(uid)
    .bind(costume_inven_index)
    .fetch_all(pool)
    .await
}

pub async fn is_activated(
    pool: &SqlitePool,
    uid: i64,
    costume_inven_index: i64,
    node_id: i32,
) -> sqlx::Result<bool> {
    let row = sqlx::query_as::<_, CostumeNodeInfo>(
        "SELECT * FROM CostumeNodeInfo WHERE Uid = ? AND CostumeInvenIndex = ? AND NodeId = ?",
    )
    .bind(uid)
    .bind(costume_inven_index)
    .bind(node_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub async fn activate(
    pool: &SqlitePool,
    uid: i64,
    costume_inven_index: i64,
    node_id: i32,
) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO CostumeNodeInfo (Uid, CostumeInvenIndex, NodeId) VALUES (?, ?, ?)",
    )
    .bind(uid)
    .bind(costume_inven_index)
    .bind(node_id)
    .execute(pool)
    .await?;
    Ok(())
}
