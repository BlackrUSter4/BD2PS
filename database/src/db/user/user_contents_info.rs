use sqlx::SqlitePool;
use serde_json::Value;
use crate::models::game::user::user_contents_info::UserContentsInfo;
/// Insert a full JSON array of UserContentsInfo records for a UID.
pub async fn insert_user_contents_info(pool: &SqlitePool, data: &Value, uid: i64) -> sqlx::Result<()> {
    let arr = match data.get("userContentsInfo").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => {
            eprintln!("insert_user_contents_info: missing or invalid 'userContentsInfo' array");
            return Ok(());
        }
    };

    for entry in arr {
        // Handle parallel repeated arrays - iterate all together
        let owner_index = entry
            .get("ownerIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let user_id = entry
            .get("userId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let title_id = entry
            .get("titleId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let portrait_costume_id = entry
            .get("portraitCostumeId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let greeting = entry
            .get("greeting")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let pvp_season = entry
            .get("pvpSeason")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let pvp_vp = entry
            .get("pvpVp")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let pvp_rank = entry
            .get("pvpRank")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let monsterhunt_rank = entry
            .get("monsterhuntRank")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let like_count = entry
            .get("likeCount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let is_all_private = entry
            .get("isAllPrivate")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let is_friend = entry
            .get("isFriend")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let room_info_index = entry
            .get("roomInfoIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let total_war_score = entry
            .get("totalWarScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let total_battle_power = entry
            .get("totalBattlePower")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let guild_base_info_index = entry
            .get("guildBaseInfoIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let my_room_like_count = entry
            .get("myRoomLikeCount")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let portrait_costume_design_id = entry
            .get("portraitCostumeDesignId")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let guild_raid_rank = entry
            .get("guildRaidRank")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let guild_raid_score = entry
            .get("guildRaidScore")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let evil_castle_greed_tower_top_floor = entry
            .get("evilCastleGreedTowerTopFloor")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let evil_castle_rage_tower_top_floor = entry
            .get("evilCastleRageTowerTopFloor")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let evil_castle_envy_tower_top_floor = entry
            .get("evilCastleEnvyTowerTopFloor")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let evil_castle_rogue_like_level = entry
            .get("evilCastleRogueLikeLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let achievement_level = entry
            .get("achievementLevel")
            .and_then(|v| v.as_i64())
            .unwrap_or_default() as i32;
        let id_card_info_index = entry
            .get("idCardInfoIndex")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let monsterhunt_rank_top_percent = entry
            .get("monsterhuntRankTopPercent")
            .and_then(|v| v.as_i64())
            .unwrap_or_default();
        let supporter_info_index = entry
            .get("supporterInfoIndex")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Get all parallel arrays
        let options_array = entry.get("options").and_then(|v| v.as_array());
        let sort_id_array = entry.get("sortId").and_then(|v| v.as_array());

        // Determine max length
        let len = 0
            .max(options_array.map(|a| a.len()).unwrap_or(0))
            .max(sort_id_array.map(|a| a.len()).unwrap_or(0));

        if len > 0 {
            // Insert one row per index (parallel iteration)
            for i in 0..len {
                let options = options_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let sort_id = sort_id_array
                    .and_then(|arr| arr.get(i))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;

                sqlx::query(
                    r#"
INSERT INTO UserContentsInfo (
    Uid,
    OwnerIndex,
    UserId,
    TitleId,
    PortraitCostumeId,
    Greeting,
    PvpSeason,
    PvpVp,
    PvpRank,
    MonsterhuntRank,
    LikeCount,
    IsAllPrivate,
    Options,
    IsFriend,
    RoomInfoIndex,
    TotalWarScore,
    TotalBattlePower,
    GuildBaseInfoIndex,
    MyRoomLikeCount,
    PortraitCostumeDesignId,
    GuildRaidRank,
    GuildRaidScore,
    EvilCastleGreedTowerTopFloor,
    EvilCastleRageTowerTopFloor,
    EvilCastleEnvyTowerTopFloor,
    EvilCastleRogueLikeLevel,
    AchievementLevel,
    SortId,
    IdCardInfoIndex,
    MonsterhuntRankTopPercent,
    SupporterInfoIndex
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
    ?
)
"#
                )
                .bind(uid)
                .bind(&owner_index)
                .bind(&user_id)
                .bind(&title_id)
                .bind(&portrait_costume_id)
                .bind(&greeting)
                .bind(&pvp_season)
                .bind(&pvp_vp)
                .bind(&pvp_rank)
                .bind(&monsterhunt_rank)
                .bind(&like_count)
                .bind(&is_all_private)
                .bind(options)
                .bind(&is_friend)
                .bind(&room_info_index)
                .bind(&total_war_score)
                .bind(&total_battle_power)
                .bind(&guild_base_info_index)
                .bind(&my_room_like_count)
                .bind(&portrait_costume_design_id)
                .bind(&guild_raid_rank)
                .bind(&guild_raid_score)
                .bind(&evil_castle_greed_tower_top_floor)
                .bind(&evil_castle_rage_tower_top_floor)
                .bind(&evil_castle_envy_tower_top_floor)
                .bind(&evil_castle_rogue_like_level)
                .bind(&achievement_level)
                .bind(sort_id)
                .bind(&id_card_info_index)
                .bind(&monsterhunt_rank_top_percent)
                .bind(&supporter_info_index)
                .execute(pool)
                .await?;
            }
        } else {
            // No items, insert one row with NULLs for repeated fields
            sqlx::query(
                r#"
INSERT INTO UserContentsInfo (
    Uid,
    OwnerIndex,
    UserId,
    TitleId,
    PortraitCostumeId,
    Greeting,
    PvpSeason,
    PvpVp,
    PvpRank,
    MonsterhuntRank,
    LikeCount,
    IsAllPrivate,
    Options,
    IsFriend,
    RoomInfoIndex,
    TotalWarScore,
    TotalBattlePower,
    GuildBaseInfoIndex,
    MyRoomLikeCount,
    PortraitCostumeDesignId,
    GuildRaidRank,
    GuildRaidScore,
    EvilCastleGreedTowerTopFloor,
    EvilCastleRageTowerTopFloor,
    EvilCastleEnvyTowerTopFloor,
    EvilCastleRogueLikeLevel,
    AchievementLevel,
    SortId,
    IdCardInfoIndex,
    MonsterhuntRankTopPercent,
    SupporterInfoIndex
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
    NULL,
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
    NULL,
    ?,
    ?,
    ?
)
"#
            )
            .bind(uid)
            .bind(&owner_index)
            .bind(&user_id)
            .bind(&title_id)
            .bind(&portrait_costume_id)
            .bind(&greeting)
            .bind(&pvp_season)
            .bind(&pvp_vp)
            .bind(&pvp_rank)
            .bind(&monsterhunt_rank)
            .bind(&like_count)
            .bind(&is_all_private)
            .bind(&is_friend)
            .bind(&room_info_index)
            .bind(&total_war_score)
            .bind(&total_battle_power)
            .bind(&guild_base_info_index)
            .bind(&my_room_like_count)
            .bind(&portrait_costume_design_id)
            .bind(&guild_raid_rank)
            .bind(&guild_raid_score)
            .bind(&evil_castle_greed_tower_top_floor)
            .bind(&evil_castle_rage_tower_top_floor)
            .bind(&evil_castle_envy_tower_top_floor)
            .bind(&evil_castle_rogue_like_level)
            .bind(&achievement_level)
            .bind(&id_card_info_index)
            .bind(&monsterhunt_rank_top_percent)
            .bind(&supporter_info_index)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}

/// Add a single UserContentsInfo record from a Rust struct.
pub async fn add_user_contents_info(pool: &SqlitePool, data: &UserContentsInfo) -> sqlx::Result<()> {
    sqlx::query(
        r#"
INSERT INTO UserContentsInfo (
    Uid,
    OwnerIndex,
    UserId,
    TitleId,
    PortraitCostumeId,
    Greeting,
    PvpSeason,
    PvpVp,
    PvpRank,
    MonsterhuntRank,
    LikeCount,
    IsAllPrivate,
    Options,
    IsFriend,
    RoomInfoIndex,
    TotalWarScore,
    TotalBattlePower,
    GuildBaseInfoIndex,
    MyRoomLikeCount,
    PortraitCostumeDesignId,
    GuildRaidRank,
    GuildRaidScore,
    EvilCastleGreedTowerTopFloor,
    EvilCastleRageTowerTopFloor,
    EvilCastleEnvyTowerTopFloor,
    EvilCastleRogueLikeLevel,
    AchievementLevel,
    SortId,
    IdCardInfoIndex,
    MonsterhuntRankTopPercent,
    SupporterInfoIndex
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
    ?
)
"#
    )
    .bind(&data.uid)
    .bind(&data.owner_index)
    .bind(&data.user_id)
    .bind(&data.title_id)
    .bind(&data.portrait_costume_id)
    .bind(&data.greeting)
    .bind(&data.pvp_season)
    .bind(&data.pvp_vp)
    .bind(&data.pvp_rank)
    .bind(&data.monsterhunt_rank)
    .bind(&data.like_count)
    .bind(&data.is_all_private)
    .bind(&data.options)
    .bind(&data.is_friend)
    .bind(&data.room_info_index)
    .bind(&data.total_war_score)
    .bind(&data.total_battle_power)
    .bind(&data.guild_base_info_index)
    .bind(&data.my_room_like_count)
    .bind(&data.portrait_costume_design_id)
    .bind(&data.guild_raid_rank)
    .bind(&data.guild_raid_score)
    .bind(&data.evil_castle_greed_tower_top_floor)
    .bind(&data.evil_castle_rage_tower_top_floor)
    .bind(&data.evil_castle_envy_tower_top_floor)
    .bind(&data.evil_castle_rogue_like_level)
    .bind(&data.achievement_level)
    .bind(&data.sort_id)
    .bind(&data.id_card_info_index)
    .bind(&data.monsterhunt_rank_top_percent)
    .bind(&data.supporter_info_index)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch all records for a given UID.
pub async fn get_user_contents_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<Vec<UserContentsInfo>> {
    sqlx::query_as::<_, UserContentsInfo>("SELECT * FROM UserContentsInfo WHERE Uid = ?")
        .bind(uid)
        .fetch_all(pool)
        .await
}

/// Delete all UserContentsInfo rows for a UID.
pub async fn delete_user_contents_info(pool: &SqlitePool, uid: i64) -> sqlx::Result<()> {
    sqlx::query("DELETE FROM UserContentsInfo WHERE Uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}