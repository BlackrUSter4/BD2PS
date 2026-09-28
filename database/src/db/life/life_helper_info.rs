use crate::models::game::life::life_helper_info::LifeHelperInfo;
use sqlx::SqlitePool;

pub async fn get_by_uid(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<LifeHelperInfo>> {
    sqlx::query_as::<_, LifeHelperInfo>("SELECT * FROM LifeHelperInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn get_by_slot(
    pool: &SqlitePool,
    uid: i64,
    helper_slot_id: i32,
) -> sqlx::Result<Option<LifeHelperInfo>> {
    sqlx::query_as::<_, LifeHelperInfo>(
        "SELECT * FROM LifeHelperInfo WHERE Uid = ? AND HelperSlotId = ?",
    )
    .bind(uid)
    .bind(helper_slot_id)
    .fetch_optional(pool)
    .await
}

pub async fn insert(pool: &SqlitePool, data: &LifeHelperInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO LifeHelperInfo (
    Uid, HelperIndex, HelperId, HelperSlotId, HelperName, UseCharId, UseHairId,
    UseHairAccessoryId, UseFaceAccessoryId, UseCostumeId, UseBodyAccessoryId,
    UseHandAccessoryId, UsePetId, UseMountId, UseEffectId, AvatarDate, WorkType, WorkId, AssignDate
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.helper_index)
    .bind(data.helper_id)
    .bind(data.helper_slot_id)
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
    .bind(data.work_type)
    .bind(data.work_id)
    .bind(data.assign_date)
    .execute(pool)
    .await?;
    Ok(result.last_insert_rowid())
}

pub async fn assign_work(
    pool: &SqlitePool,
    index: i64,
    work_type: i32,
    work_id: i32,
    assign_date: i64,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE LifeHelperInfo SET WorkType = ?, WorkId = ?, AssignDate = ? WHERE \"Index\" = ?",
    )
    .bind(work_type)
    .bind(work_id)
    .bind(assign_date)
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn clear_work(pool: &SqlitePool, index: i64) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE LifeHelperInfo SET WorkType = NULL, WorkId = NULL, AssignDate = NULL WHERE \"Index\" = ?",
    )
    .bind(index)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn rename(pool: &SqlitePool, index: i64, name: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE LifeHelperInfo SET HelperName = ? WHERE \"Index\" = ?")
        .bind(name)
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_index(pool: &SqlitePool, index: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM LifeHelperInfo WHERE \"Index\" = ?")
        .bind(index)
        .execute(pool)
        .await?;
    Ok(())
}
