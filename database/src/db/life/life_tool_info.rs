use crate::models::game::life::life_tool_info::LifeToolInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeToolInfo>> {
    sqlx::query_as::<_, LifeToolInfo>("SELECT * FROM LifeToolInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Insert a group's first tool, or upgrade it to a new tool id if the group already exists.
pub async fn upsert(pool: &SqlitePool, uid: i64, group_id: i32, tool_id: i32) -> sqlx::Result<()> {
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT \"Index\" FROM LifeToolInfo WHERE Uid = ? AND GroupId = ?")
            .bind(uid)
            .bind(group_id)
            .fetch_optional(pool)
            .await?;

    if let Some((index,)) = existing {
        sqlx::query("UPDATE LifeToolInfo SET ToolId = ? WHERE \"Index\" = ?")
            .bind(tool_id)
            .bind(index)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO LifeToolInfo (Uid, GroupId, ToolId) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(group_id)
            .bind(tool_id)
            .execute(pool)
            .await?;
    }
    Ok(())
}
