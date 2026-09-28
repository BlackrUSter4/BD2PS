use crate::models::game::chat::chat_setting_info::ChatSettingInfo;
use sqlx::SqlitePool;

pub async fn get_or_default(pool: &SqlitePool, uid: i64) -> ChatSettingInfo {
    sqlx::query_as::<_, ChatSettingInfo>("SELECT * FROM ChatSettingInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(ChatSettingInfo {
            uid,
            auto_translate_flag: 0,
            global_chat_flag: 0,
            visual_flag: 0,
        })
}

pub async fn save(pool: &SqlitePool, info: &ChatSettingInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO ChatSettingInfo (Uid, AutoTranslateFlag, GlobalChatFlag, VisualFlag)
VALUES (?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    AutoTranslateFlag = excluded.AutoTranslateFlag,
    GlobalChatFlag = excluded.GlobalChatFlag,
    VisualFlag = excluded.VisualFlag
"#,
    )
    .bind(info.uid)
    .bind(info.auto_translate_flag)
    .bind(info.global_chat_flag)
    .bind(info.visual_flag)
    .execute(pool)
    .await?;
    Ok(())
}
