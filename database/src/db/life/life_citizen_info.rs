use crate::models::game::life::life_citizen_info::LifeCitizenInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeCitizenInfo>> {
    sqlx::query_as::<_, LifeCitizenInfo>("SELECT * FROM LifeCitizenInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_slot(
    pool: &SqlitePool,
    uid: i64,
    citizen_slot_id: i32,
) -> sqlx::Result<Option<LifeCitizenInfo>> {
    sqlx::query_as::<_, LifeCitizenInfo>(
        "SELECT * FROM LifeCitizenInfo WHERE Uid = ? AND CitizenSlotId = ?",
    )
    .bind(uid)
    .bind(citizen_slot_id)
    .fetch_optional(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, data: &LifeCitizenInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO LifeCitizenInfo (
    Uid, CitizenIndex, CitizenSlotId, UseCharId, UseHairId, UseHairAccessoryId,
    UseFaceAccessoryId, UseCostumeId, UseBodyAccessoryId, UseHandAccessoryId,
    UsePetId, UseMountId, UseEffectId, AvatarDate
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.citizen_index)
    .bind(data.citizen_slot_id)
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

pub async fn update_avatar(pool: &SqlitePool, data: &LifeCitizenInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
UPDATE LifeCitizenInfo SET
    UseCharId = ?, UseHairId = ?, UseHairAccessoryId = ?, UseFaceAccessoryId = ?,
    UseCostumeId = ?, UseBodyAccessoryId = ?, UseHandAccessoryId = ?, UsePetId = ?,
    UseMountId = ?, UseEffectId = ?, AvatarDate = ?
WHERE "Index" = ?
"#,
    )
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
    .bind(data.index)
    .execute(pool)
    .await?;
    Ok(())
}
