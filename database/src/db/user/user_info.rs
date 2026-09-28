use crate::models::game::user::user_info::UserInfo;
use sqlx::SqlitePool;

/// Add a single UserInfo record from a Rust struct.
pub async fn add_user_info(pool: &SqlitePool, data: &UserInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO UserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserKey,
    LastPlayPackId,
    InvenSlot,
    StorageSlot,
    Gold,
    FreeJewelry,
    Jewelry,
    EquipSlot,
    Catalyst,
    Exp,
    LevelReward,
    PortraitCostumeId,
    EquipStorageSlot,
    PvpTicket,
    Medal,
    EvilCastleCoin,
    UserType,
    FreeHuntingAp,
    BonusHuntingAp,
    PvpTicketStack,
    Mileage,
    HopePowder,
    UnregDate,
    PurchaseCountInfoIndex,
    IsFirstGacha,
    PresetSlot,
    BlockDate,
    ReturnStatusEndTime,
    MyRoomSlot,
    TotalWarPresetSlot,
    EventApFree,
    EventApStack,
    FreeTorchLightAp,
    TorchLightAp,
    JoinTime,
    NewbiePassStep,
    BlockReason,
    RogueLikeAp,
    RogueLikeApStack,
    LoginDate,
    GuildCoin,
    CafeteriaCoin,
    DatingApFree,
    DatingApStack,
    MiniGameCoin,
    CanCharAutoRevive,
    AutoReviveCastingCharInvenIndex,
    RecommendDeckUserOptionInfoIndex,
    MonsterHuntPresetSlot,
    GuildRaidPresetSlot,
    MyRoomDecoCoin,
    PortraitCostumeDesignId
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.user_key)
    .bind(&data.last_play_pack_id)
    .bind(&data.inven_slot)
    .bind(&data.storage_slot)
    .bind(&data.gold)
    .bind(&data.free_jewelry)
    .bind(&data.jewelry)
    .bind(&data.equip_slot)
    .bind(&data.catalyst)
    .bind(&data.exp)
    .bind(&data.level_reward)
    .bind(&data.portrait_costume_id)
    .bind(&data.equip_storage_slot)
    .bind(&data.pvp_ticket)
    .bind(&data.medal)
    .bind(&data.evil_castle_coin)
    .bind(&data.user_type)
    .bind(&data.free_hunting_ap)
    .bind(&data.bonus_hunting_ap)
    .bind(&data.pvp_ticket_stack)
    .bind(&data.mileage)
    .bind(&data.hope_powder)
    .bind(&data.unreg_date)
    .bind(&data.purchase_count_info_index)
    .bind(&data.is_first_gacha)
    .bind(&data.preset_slot)
    .bind(&data.block_date)
    .bind(&data.return_status_end_time)
    .bind(&data.my_room_slot)
    .bind(&data.total_war_preset_slot)
    .bind(&data.event_ap_free)
    .bind(&data.event_ap_stack)
    .bind(&data.free_torch_light_ap)
    .bind(&data.torch_light_ap)
    .bind(&data.join_time)
    .bind(&data.newbie_pass_step)
    .bind(&data.block_reason)
    .bind(&data.rogue_like_ap)
    .bind(&data.rogue_like_ap_stack)
    .bind(&data.login_date)
    .bind(&data.guild_coin)
    .bind(&data.cafeteria_coin)
    .bind(&data.dating_ap_free)
    .bind(&data.dating_ap_stack)
    .bind(&data.mini_game_coin)
    .bind(&data.can_char_auto_revive)
    .bind(&data.auto_revive_casting_char_inven_index)
    .bind(&data.recommend_deck_user_option_info_index)
    .bind(&data.monster_hunt_preset_slot)
    .bind(&data.guild_raid_preset_slot)
    .bind(&data.my_room_deco_coin)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<UserInfo>> {
    sqlx::query_as::<_, UserInfo>("SELECT * FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all UserInfo rows for a UID.
pub async fn delete_user_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get a single record by Index (rowid)
pub async fn get_by_index(pool: &SqlitePool, uid: i64, index: i64) -> sqlx::Result<UserInfo> {
    sqlx::query_as::<_, UserInfo>("SELECT * FROM UserInfo WHERE Uid = ? AND Index = ?")
        .bind(uid)
        .bind(index)
        .fetch_one(pool)
        .await
}

/// Get multiple records by their Index (rowids)
pub async fn get_all_by_index(
    pool: &SqlitePool,
    uid: i64,
    index: &[i64],
) -> sqlx::Result<Vec<UserInfo>> {
    if index.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = index.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!(
        "SELECT * FROM UserInfo WHERE Uid = ? AND Index IN ({})",
        placeholders
    );

    let mut query = sqlx::query_as::<_, UserInfo>(&query).bind(uid);
    for idx in index {
        query = query.bind(idx);
    }

    query.fetch_all(pool).await
}

/// Insert and return the rowid (Index)
pub async fn insert(pool: &SqlitePool, data: &UserInfo) -> sqlx::Result<i64> {
    let result = sqlx::query(
        r#"
INSERT INTO UserInfo (
    Uid,
    OwnerIndex,
    UserId,
    UserKey,
    LastPlayPackId,
    InvenSlot,
    StorageSlot,
    Gold,
    FreeJewelry,
    Jewelry,
    EquipSlot,
    Catalyst,
    Exp,
    LevelReward,
    PortraitCostumeId,
    EquipStorageSlot,
    PvpTicket,
    Medal,
    EvilCastleCoin,
    UserType,
    FreeHuntingAp,
    BonusHuntingAp,
    PvpTicketStack,
    Mileage,
    HopePowder,
    UnregDate,
    PurchaseCountInfoIndex,
    IsFirstGacha,
    PresetSlot,
    BlockDate,
    ReturnStatusEndTime,
    MyRoomSlot,
    TotalWarPresetSlot,
    EventApFree,
    EventApStack,
    FreeTorchLightAp,
    TorchLightAp,
    JoinTime,
    NewbiePassStep,
    BlockReason,
    RogueLikeAp,
    RogueLikeApStack,
    LoginDate,
    GuildCoin,
    CafeteriaCoin,
    DatingApFree,
    DatingApStack,
    MiniGameCoin,
    CanCharAutoRevive,
    AutoReviveCastingCharInvenIndex,
    RecommendDeckUserOptionInfoIndex,
    MonsterHuntPresetSlot,
    GuildRaidPresetSlot,
    MyRoomDecoCoin,
    PortraitCostumeDesignId
) VALUES (
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?,
    ?
)
"#,
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.user_key)
    .bind(&data.last_play_pack_id)
    .bind(&data.inven_slot)
    .bind(&data.storage_slot)
    .bind(&data.gold)
    .bind(&data.free_jewelry)
    .bind(&data.jewelry)
    .bind(&data.equip_slot)
    .bind(&data.catalyst)
    .bind(&data.exp)
    .bind(&data.level_reward)
    .bind(&data.portrait_costume_id)
    .bind(&data.equip_storage_slot)
    .bind(&data.pvp_ticket)
    .bind(&data.medal)
    .bind(&data.evil_castle_coin)
    .bind(&data.user_type)
    .bind(&data.free_hunting_ap)
    .bind(&data.bonus_hunting_ap)
    .bind(&data.pvp_ticket_stack)
    .bind(&data.mileage)
    .bind(&data.hope_powder)
    .bind(&data.unreg_date)
    .bind(&data.purchase_count_info_index)
    .bind(&data.is_first_gacha)
    .bind(&data.preset_slot)
    .bind(&data.block_date)
    .bind(&data.return_status_end_time)
    .bind(&data.my_room_slot)
    .bind(&data.total_war_preset_slot)
    .bind(&data.event_ap_free)
    .bind(&data.event_ap_stack)
    .bind(&data.free_torch_light_ap)
    .bind(&data.torch_light_ap)
    .bind(&data.join_time)
    .bind(&data.newbie_pass_step)
    .bind(&data.block_reason)
    .bind(&data.rogue_like_ap)
    .bind(&data.rogue_like_ap_stack)
    .bind(&data.login_date)
    .bind(&data.guild_coin)
    .bind(&data.cafeteria_coin)
    .bind(&data.dating_ap_free)
    .bind(&data.dating_ap_stack)
    .bind(&data.mini_game_coin)
    .bind(&data.can_char_auto_revive)
    .bind(&data.auto_revive_casting_char_inven_index)
    .bind(&data.recommend_deck_user_option_info_index)
    .bind(&data.monster_hunt_preset_slot)
    .bind(&data.guild_raid_preset_slot)
    .bind(&data.my_room_deco_coin)
    .bind(&data.portrait_costume_design_id)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update just the account's greeting message.
pub async fn update_greeting(pool: &SqlitePool, uid: i64, greeting: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET Greeting = ? WHERE Uid = ?")
        .bind(greeting)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update just the account's selected title.
pub async fn update_title(pool: &SqlitePool, uid: i64, title_id: i32) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET TitleId = ? WHERE Uid = ?")
        .bind(title_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update just the account's nickname (UserId doubles as the display nickname here).
pub async fn update_nickname(pool: &SqlitePool, uid: i64, user_id: &str) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET UserId = ? WHERE Uid = ?")
        .bind(user_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update just the account's portrait costume + design id.
pub async fn set_unreg_date(pool: &SqlitePool, uid: i64, unreg_date: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET UnregDate = ? WHERE Uid = ?")
        .bind(unreg_date)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_contents_info_option(pool: &SqlitePool, uid: i64, is_all_private: bool, options: &[i32]) -> sqlx::Result<()> {
    let options_json = serde_json::to_string(options).unwrap_or_default();
    sqlx::query("UPDATE UserInfo SET IsAllPrivate = ?, PrivacyOptions = ? WHERE Uid = ?")
        .bind(is_all_private as i32)
        .bind(options_json)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn clear_unreg_date(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET UnregDate = NULL WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_inven_slot(pool: &SqlitePool, uid: i64, add_slot: i32) -> sqlx::Result<i32> {
    sqlx::query("UPDATE UserInfo SET InvenSlot = COALESCE(InvenSlot, 0) + ? WHERE Uid = ?")
        .bind(add_slot)
        .bind(uid)
        .execute(pool)
        .await?;
    let slot: Option<i32> = sqlx::query_scalar("SELECT InvenSlot FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await?
        .flatten();
    Ok(slot.unwrap_or(0))
}

pub async fn add_storage_slot(pool: &SqlitePool, uid: i64, add_slot: i32) -> sqlx::Result<i32> {
    sqlx::query("UPDATE UserInfo SET StorageSlot = COALESCE(StorageSlot, 0) + ? WHERE Uid = ?")
        .bind(add_slot)
        .bind(uid)
        .execute(pool)
        .await?;
    let slot: Option<i32> = sqlx::query_scalar("SELECT StorageSlot FROM UserInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_optional(pool)
        .await?
        .flatten();
    Ok(slot.unwrap_or(0))
}

pub async fn update_portrait(
    pool: &SqlitePool,
    uid: i64,
    portrait_costume_id: i32,
    portrait_costume_design_id: Option<i32>,
) -> sqlx::Result<()> {
    sqlx::query("UPDATE UserInfo SET PortraitCostumeId = ?, PortraitCostumeDesignId = ? WHERE Uid = ?")
        .bind(portrait_costume_id)
        .bind(portrait_costume_design_id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}
