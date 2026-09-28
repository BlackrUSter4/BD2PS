use crate::models::game::fishing::fishing_user_info::FishingUserInfo;
use sqlx::SqlitePool;

pub async fn get(pool: &SqlitePool, uid: i64) -> sqlx::Result<Option<FishingUserInfo>> {
    sqlx::query_as::<_, FishingUserInfo>("SELECT * FROM FishingUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn get_or_create(pool: &SqlitePool, uid: i64) -> sqlx::Result<FishingUserInfo> {
    if let Some(existing) = get(pool, uid).await? {
        return Ok(existing);
    }
    let data = FishingUserInfo { uid, ..Default::default() };
    insert(pool, &data).await?;
    Ok(data)
}

pub async fn insert(pool: &SqlitePool, data: &FishingUserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO FishingUserInfo (
    Uid, Exp, Level, BoatLevel, BoatSkinId, UseRodInvenIndex, MultiApResetTime,
    TrapRewardReceiptTime, FishInvenSlotCount
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
    )
    .bind(data.uid)
    .bind(data.exp)
    .bind(data.level)
    .bind(data.boat_level)
    .bind(data.boat_skin_id)
    .bind(data.use_rod_inven_index)
    .bind(data.multi_ap_reset_time)
    .bind(data.trap_reward_receipt_time)
    .bind(data.fish_inven_slot_count)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_exp(pool: &SqlitePool, uid: i64, exp_delta: i32, level: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET Exp = Exp + ?, Level = ? WHERE Uid = ?")
        .bind(exp_delta)
        .bind(level)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_boat_level(pool: &SqlitePool, uid: i64, level: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET BoatLevel = ? WHERE Uid = ?")
        .bind(level)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_boat_skin(pool: &SqlitePool, uid: i64, skin_id: i32) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET BoatSkinId = ? WHERE Uid = ?")
        .bind(skin_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_use_rod(pool: &SqlitePool, uid: i64, rod_inven_index: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET UseRodInvenIndex = ? WHERE Uid = ?")
        .bind(rod_inven_index)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_multi_ap_reset_time(pool: &SqlitePool, uid: i64, time: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET MultiApResetTime = ? WHERE Uid = ?")
        .bind(time)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_trap_reward_receipt_time(pool: &SqlitePool, uid: i64, time: i64) -> sqlx::Result<()> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET TrapRewardReceiptTime = ? WHERE Uid = ?")
        .bind(time)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_fish_inven_slot(pool: &SqlitePool, uid: i64, add_slot: i32) -> sqlx::Result<i32> {
    get_or_create(pool, uid).await?;
    sqlx::query("UPDATE FishingUserInfo SET FishInvenSlotCount = FishInvenSlotCount + ? WHERE Uid = ?")
        .bind(add_slot)
        .bind(uid)
        .execute(pool)
        .await?;
    let row: (i32,) = sqlx::query_as("SELECT FishInvenSlotCount FROM FishingUserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}
