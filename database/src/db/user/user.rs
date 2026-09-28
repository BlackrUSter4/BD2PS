use crate::db::starter::starter_data::load_all_starter_data;
use crate::models::game::user::user_info::UserInfo;
use crate::models::user::account::Account;
use sqlx::SqlitePool;

pub async fn find_account(pool: &SqlitePool, uid: i64) -> Result<Option<Account>, sqlx::Error> {
    sqlx::query_as::<_, Account>("SELECT Uid, UserName, Password FROM Account WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn find_user_info(pool: &SqlitePool, uid: i64) -> Result<Option<UserInfo>, sqlx::Error> {
    sqlx::query_as::<_, UserInfo>("SELECT * FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn get_next_owner_index(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let result: Option<(i64,)> =
        sqlx::query_as("SELECT COALESCE(MAX(OwnerIndex), 0) + 1 FROM UserInfo")
            .fetch_optional(pool)
            .await?;
    Ok(result.map(|(idx,)| idx).unwrap_or(1))
}

pub async fn get_user_by_uid(
    pool: &SqlitePool,
    uid: i64,
) -> Result<Option<(Account, UserInfo)>, sqlx::Error> {
    let account = match find_account(pool, uid).await? {
        Some(a) => a,
        None => return Ok(None),
    };

    let user_info = match find_user_info(pool, uid).await? {
        Some(u) => u,
        None => return Ok(None),
    };

    Ok(Some((account, user_info)))
}

pub async fn get_or_create_user(
    pool: &SqlitePool,
    uid: i64,
) -> Result<(Account, UserInfo), sqlx::Error> {
    if let Some((account, user_info)) = get_user_by_uid(pool, uid).await? {
        return Ok((account, user_info));
    }
    create_default_user(pool, uid).await
}

pub async fn create_default_user(
    pool: &SqlitePool,
    uid: i64,
) -> Result<(Account, UserInfo), sqlx::Error> {
    use chrono::Utc;

    let mut tx = pool.begin().await?;

    // Get the next owner index
    let owner_index = get_next_owner_index(pool).await?;

    let username = format!("Guest_{}", owner_index);
    let now_ms = Utc::now().timestamp_millis();

    // Insert account FIRST
    sqlx::query("INSERT INTO Account (Uid, UserName, Password) VALUES (?, ?, ?)")
        .bind(uid)
        .bind(&username)
        .bind("123456")
        .execute(&mut *tx)
        .await?;

    // Insert user info with both Uid and OwnerIndex
    sqlx::query(
        r#"INSERT INTO UserInfo (
            Uid, OwnerIndex, UserId, LastPlayPackId, InvenSlot, StorageSlot,
            Gold, FreeJewelry, Jewelry, EquipSlot, Catalyst, Exp,
            LevelReward, PortraitCostumeId, EquipStorageSlot, PvPTicket,
            Medal, EvilCastleCoin, UserType, FreeHuntingAP, BonusHuntingAP,
            PvPTicketStack, Mileage, HopePowder, UnregDate, IsFirstGacha,
            PresetSlot, BlockDate, ReturnStatusEndTime, MyRoomSlot,
            TotalWarPresetSlot, EventApFree, EventApStack, FreeTorchLightAp,
            TorchLightAp, JoinTime, NewbiePassStep, BlockReason,
            RogueLikeAp, RogueLikeApStack, LoginDate
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
        .bind(uid)                      // Uid - the actual user ID
        .bind(owner_index)              // OwnerIndex - generated sequence
        .bind(&username)                // UserId
        .bind(1)                        // LastPlayPackId
        .bind(100)                      // InvenSlot
        .bind(100)                      // StorageSlot
        .bind(12500_i64)                // Gold
        .bind(150_i64)                  // FreeJewelry
        .bind(0_i64)                    // Jewelry
        .bind(500)                      // EquipSlot
        .bind(200_i64)                  // Catalyst
        .bind(0)                        // Exp
        .bind(1)                        // LevelReward
        .bind(101)                      // PortraitCostumeId
        .bind(100)                      // EquipStorageSlot
        .bind(40)                       // PvPTicket
        .bind(0)                        // Medal
        .bind(0)                        // EvilCastleCoin
        .bind(0)                        // UserType
        .bind(60)                       // FreeHuntingAP
        .bind(0)                        // BonusHuntingAP
        .bind(0)                        // PvPTicketStack
        .bind(0)                        // Mileage
        .bind(0)                        // HopePowder
        .bind(0_i64)                    // UnregDate
        .bind(0)                        // IsFirstGacha
        .bind(5)                        // PresetSlot
        .bind(0_i64)                    // BlockDate
        .bind(0_i64)                    // ReturnStatusEndTime
        .bind(3)                        // MyRoomSlot
        .bind(5)                        // TotalWarPresetSlot
        .bind(5)                        // EventApFree
        .bind(0)                        // EventApStack
        .bind(60)                       // FreeTorchLightAp
        .bind(0)                        // TorchLightAp
        .bind(now_ms)                   // JoinTime
        .bind(1)                        // NewbiePassStep
        .bind(0)                        // BlockReason
        .bind(3)                        // RogueLikeAp
        .bind(0)                        // RogueLikeApStack
        .bind(now_ms)                   // LoginDate
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    // Load all starter data (items, characters, etc.)
    load_all_starter_data(pool, uid).await?;

    // Fetch and return created records
    let account = find_account(pool, uid).await?.unwrap();
    let user_info = find_user_info(pool, uid).await?.unwrap();

    Ok((account, user_info))
}

pub async fn update_return_status(
    pool: &SqlitePool,
    uid: i64,
    timestamp: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE UserInfo SET ReturnStatusEndTime = ? WHERE Uid = ?")
        .bind(timestamp)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
