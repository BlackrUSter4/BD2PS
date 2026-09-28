use crate::models::game::avatar::avatar_use_info::AvatarUseInfo;
use sqlx::SqlitePool;

/// Fetch the account's current avatar loadout, or a fresh default (nothing equipped) if
/// it has never been saved before — this project has no `AvatarDefaultTable` data captured
/// to seed a "real" starter loadout from, so an all-empty default is the honest choice.
pub async fn get_or_default(pool: &SqlitePool, uid: i64) -> AvatarUseInfo {
    sqlx::query_as::<_, AvatarUseInfo>("SELECT * FROM AvatarUseInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .unwrap_or(AvatarUseInfo {
            uid,
            ..Default::default()
        })
}

#[allow(clippy::too_many_arguments)]
pub async fn save(pool: &SqlitePool, info: &AvatarUseInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO AvatarUseInfo (
    Uid, UseCharId, UseHairId, UseHairAccessoryId, UseFaceAccessoryId, UseCostumeId,
    UseBodyAccessoryId, UseHandAccessoryId, UsePetId, UseMountId, UseEffectId, Date
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
ON CONFLICT(Uid) DO UPDATE SET
    UseCharId = excluded.UseCharId,
    UseHairId = excluded.UseHairId,
    UseHairAccessoryId = excluded.UseHairAccessoryId,
    UseFaceAccessoryId = excluded.UseFaceAccessoryId,
    UseCostumeId = excluded.UseCostumeId,
    UseBodyAccessoryId = excluded.UseBodyAccessoryId,
    UseHandAccessoryId = excluded.UseHandAccessoryId,
    UsePetId = excluded.UsePetId,
    UseMountId = excluded.UseMountId,
    UseEffectId = excluded.UseEffectId,
    Date = excluded.Date
"#,
    )
    .bind(info.uid)
    .bind(info.use_char_id)
    .bind(info.use_hair_id)
    .bind(info.use_hair_accessory_id)
    .bind(info.use_face_accessory_id)
    .bind(info.use_costume_id)
    .bind(info.use_body_accessory_id)
    .bind(info.use_hand_accessory_id)
    .bind(info.use_pet_id)
    .bind(info.use_mount_id)
    .bind(info.use_effect_id)
    .bind(info.date)
    .execute(pool)
    .await?;
    Ok(())
}
