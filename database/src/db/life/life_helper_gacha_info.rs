use crate::models::game::life::life_helper_gacha_info::LifeHelperGachaInfo;
use sqlx::SqlitePool;

pub async fn get_by_slot(
    pool: &SqlitePool,
    uid: i64,
    helper_slot_id: i32,
) -> sqlx::Result<Vec<LifeHelperGachaInfo>> {
    sqlx::query_as::<_, LifeHelperGachaInfo>(
        "SELECT * FROM LifeHelperGachaInfo WHERE Uid = ? AND HelperSlotId = ?",
    )
    .bind(uid)
    .bind(helper_slot_id)
    .fetch_all(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, data: &LifeHelperGachaInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO LifeHelperGachaInfo (
    Uid, HelperSlotId, HelperId, HelperName, UseCharId, UseHairId, UseHairAccessoryId,
    UseFaceAccessoryId, UseCostumeId, UseBodyAccessoryId, UseHandAccessoryId,
    UsePetId, UseMountId, UseEffectId, AvatarDate
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.helper_slot_id)
    .bind(data.helper_id)
    .bind(&data.helper_name)
    .bind(data.use_char_id)
    .bind(data.use_hair_id)
    .bind(data.use_hair_accessory_id)
    .bind(data.use_face_accessory_id)
    .bind(data.use_costume_id)
    .bind(data.use_body_accessory_id)
    .bind(data.use_hand_accessory_id)
    .bind(data.use_pet_id)
    .bind(data.use_mount_id)
    .bind(data.use_effect_id)
    .bind(data.avatar_date)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

/// Clear the gacha candidate pool for a helper slot (e.g. before rerolling, or on delete).
pub async fn clear_slot(pool: &SqlitePool, uid: i64, helper_slot_id: i32) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeHelperGachaInfo WHERE Uid = ? AND HelperSlotId = ?")
        .bind(uid)
        .bind(helper_slot_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_index(pool: &SqlitePool, index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeHelperGachaInfo WHERE \"Index\" = ?")
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}
