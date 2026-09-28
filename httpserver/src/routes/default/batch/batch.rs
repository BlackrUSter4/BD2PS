use actix_web::{HttpResponse, Result, put, web};
use bd2::proto::proto_net::*;
use crypto::aestools::AesTools;
use crypto::network::{GameResponse, parse_packet};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::{error, info, warn};

#[derive(Debug, Deserialize)]
struct BatchRequestModel {
    path: String,
    #[serde(rename = "requestData")]
    request_data: String,
}

#[derive(Debug, Serialize)]
struct BatchResponseItem {
    path: String,
    #[serde(rename = "responseData")]
    response_data: GameResponse,
}

#[put("BatchRequest")]
pub async fn batch_request_handler(
    pool: web::Data<SqlitePool>,
    body: web::Bytes,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    info!("Received batch request");

    let body_str = std::str::from_utf8(&body).map_err(|e| {
        error!("Failed to convert body to UTF-8: {}", e);
        actix_web::error::ErrorBadRequest("Invalid UTF-8 in body")
    })?;

    let decrypted_json = match decrypt_batch_request(body_str) {
        Ok(json) => json,
        Err(e) => {
            warn!("Failed to decrypt batch request: {}", e);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "decryption_failed"
            })));
        }
    };

    let batch_requests: Vec<BatchRequestModel> = match serde_json::from_str(&decrypted_json) {
        Ok(r) => r,
        Err(e) => {
            warn!("Failed to parse batch requests: {}", e);
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "invalid_json"
            })));
        }
    };

    let uid = *user_id;
    let mut responses = Vec::new();

    info!(
        "Processing {} batch requests for user {}",
        batch_requests.len(),
        uid
    );

    for request in batch_requests {
        let method_name = request.path.trim_start_matches('/');
        info!("Processing batch method: {}", method_name);

        let game_response = match method_name {
            "MailInfo" => {
                match parse_packet::<MailInfoRequest>("MailInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::mail::mail_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse MailInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CashMailInfo" => {
                match parse_packet::<CashMailInfoRequest>("CashMailInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::cash::cash_mail_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse CashMailInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PackInfo" => {
                match parse_packet::<PackInfoRequest>("PackInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::pack::pack_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse PackInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "QuestMaxClearInfo" => {
                match parse_packet::<QuestMaxClearInfoRequest>(
                    "QuestMaxClearInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::quest::quest_max_clear_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse QuestMaxClearInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PassInfo" => {
                match parse_packet::<PassInfoRequest>("PassInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::pass::pass_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse PassInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PvpBattleUserInfo" => {
                match parse_packet::<PvpBattleUserInfoRequest>(
                    "PvpBattleUserInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::pvp::pvp_battle_user_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse PvpBattleUserInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "ItemInfo" => {
                match parse_packet::<ItemInfoRequest>("ItemInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::item::item_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse ItemInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CostumeInfo" => {
                match parse_packet::<CostumeInfoRequest>("CostumeInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::costume::costume_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse CostumeInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "RecipeInfo" => {
                match parse_packet::<RecipeInfoRequest>("RecipeInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::recipe::recipe_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse RecipeInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CashShopInfo" => {
                match parse_packet::<CashShopInfoRequest>("CashShopInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::cash::cash_shop_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse CashShopInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "TutorialInfo" => {
                match parse_packet::<TutorialInfoRequest>("TutorialInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::tutorial::tutorial_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse TutorialInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CharInfo" => {
                match parse_packet::<CharInfoRequest>("CharInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::char::char_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse CharInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "DispatchInfo" => {
                match parse_packet::<DispatchInfoRequest>("DispatchInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::dispatch::dispatch_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse DispatchInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "TotalWarRewardState" => {
                match parse_packet::<TotalWarRewardStateRequest>(
                    "TotalWarRewardState",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::total::total_war_reward_state::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse TotalWarRewardState: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CommunityRewardInfo" => {
                match parse_packet::<CommunityRewardInfoRequest>(
                    "CommunityRewardInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::community::community_reward_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse CommunityRewardInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EventScheduleInfo" => {
                match parse_packet::<EventScheduleInfoRequest>(
                    "EventScheduleInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::event::event_schedule_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse EventScheduleInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EventExchangeInfo" => {
                match parse_packet::<EventExchangeInfoRequest>(
                    "EventExchangeInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::event::event_exchange_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse EventExchangeInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MonsterHuntScheduleInfo" => {
                match parse_packet::<MonsterHuntScheduleInfoRequest>(
                    "MonsterHuntScheduleInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::monster::monster_hunt_schedule_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse MonsterHuntScheduleInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MonsterHuntDeckInfo" => {
                match parse_packet::<MonsterHuntDeckInfoRequest>(
                    "MonsterHuntDeckInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::monster::monster_hunt_deck_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse MonsterHuntDeckInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CharScoutInfo" => {
                match parse_packet::<CharScoutInfoRequest>("CharScoutInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::char::char_scout_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse CharScoutInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "AchievementInfo" => {
                match parse_packet::<AchievementInfoRequest>(
                    "AchievementInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::achievement::achievement_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse AchievementInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CharPartnerInfo" => {
                match parse_packet::<CharPartnerInfoRequest>(
                    "CharPartnerInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::char::char_partner_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse CharPartnerInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PresetInfo" => {
                match parse_packet::<PresetInfoRequest>("PresetInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::preset::preset_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse PresetInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "HuntDispatchInfo" => {
                match parse_packet::<HuntDispatchInfoRequest>(
                    "HuntDispatchInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::hunt::hunt_dispatch_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse HuntDispatchInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MonsterHuntUserInfo" => {
                match parse_packet::<MonsterHuntUserInfoRequest>(
                    "MonsterHuntUserInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::monster::monster_hunt_user_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse MonsterHuntUserInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PersonalInfo" => {
                match parse_packet::<PersonalInfoRequest>("PersonalInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::personal::personal_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse PersonalInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MyLikeInfo" => {
                match parse_packet::<MyLikeInfoRequest>("MyLikeInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::my::my_like_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse MyLikeInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "FriendInfoList" => {
                match parse_packet::<FriendInfoListRequest>("FriendInfoList", &request.request_data)
                {
                    Ok(req) => {
                        gameserver::logic::game::friend::friend_info_list::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse FriendInfoList: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EventHubInfo" => {
                match parse_packet::<EventHubInfoRequest>("EventHubInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::event::event_hub_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse EventHubInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MiniGameHubInfo" => {
                match parse_packet::<MiniGameHubInfoRequest>(
                    "MiniGameHubInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::mini::mini_game_hub_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse MiniGameHubInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EquipPresetInfo" => {
                match parse_packet::<EquipPresetInfoRequest>(
                    "EquipPresetInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::equip::equip_preset_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse EquipPresetInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MyRoomItemInfo" => {
                match parse_packet::<MyRoomItemInfoRequest>("MyRoomItemInfo", &request.request_data)
                {
                    Ok(req) => {
                        gameserver::logic::game::my::my_room_item_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse MyRoomItemInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "SeasonRewardInfo" => {
                match parse_packet::<SeasonRewardInfoRequest>(
                    "SeasonRewardInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::season::season_reward_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse SeasonRewardInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "GuildInitInfo" => {
                match parse_packet::<GuildInitInfoRequest>("GuildInitInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::guild::guild_init_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse GuildInitInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CharAwakeInfo" => {
                match parse_packet::<CharAwakeInfoRequest>("CharAwakeInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::char::char_awake_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse CharAwakeInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "RootSortIdInfo" => {
                match parse_packet::<RootSortIdInfoRequest>("RootSortIdInfo", &request.request_data)
                {
                    Ok(req) => {
                        gameserver::logic::game::root::root_sort_id_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse RootSortIdInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "DatingInfo" => {
                match parse_packet::<DatingInfoRequest>("DatingInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::dating::dating_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse DatingInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "GuildRaidSeasonReward" => {
                match parse_packet::<GuildRaidSeasonRewardRequest>(
                    "GuildRaidSeasonReward",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::guild::guild_raid_season_reward::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse GuildRaidSeasonReward: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EvilCastleTowerInfo" => {
                match parse_packet::<EvilCastleTowerInfoRequest>(
                    "EvilCastleTowerInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::evil::evil_castle_tower_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse EvilCastleTowerInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "CafeteriaInfo" => {
                match parse_packet::<CafeteriaInfoRequest>("CafeteriaInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::cafeteria::cafeteria_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse CafeteriaInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "MiniGameDefenseInfo" => {
                match parse_packet::<MiniGameDefenseInfoRequest>(
                    "MiniGameDefenseInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::mini::mini_game_defense_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse MiniGameDefenseInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "DeckCostumeSettingInfo" => {
                match parse_packet::<DeckCostumeSettingInfoRequest>(
                    "DeckCostumeSettingInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::deck::deck_costume_setting_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse DeckCostumeSettingInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PrestigeSkinInfo" => {
                match parse_packet::<PrestigeSkinInfoRequest>(
                    "PrestigeSkinInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::prestige::prestige_skin_info::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse PrestigeSkinInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "EvilCastleDailyRewardState" => {
                match parse_packet::<EvilCastleDailyRewardStateRequest>(
                    "EvilCastleDailyRewardState",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::evil::evil_castle_daily_reward_state::handle(
                            &pool, uid, req,
                        )
                        .await
                    }
                    Err(e) => {
                        error!("Failed to parse EvilCastleDailyRewardState: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "IdCardPresetInfo" => {
                match parse_packet::<IdCardPresetInfoRequest>(
                    "IdCardPresetInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::id::id_card_preset_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse IdCardPresetInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "SkyWayScheduleInfo" => {
                match parse_packet::<SkyWayScheduleInfoRequest>(
                    "SkyWayScheduleInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::sky::sky_way_schedule_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse SkyWayScheduleInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "FieldTrapInfo" => {
                match parse_packet::<FieldTrapInfoRequest>("FieldTrapInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::field::field_trap_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse FieldTrapInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "DeckInfo" => {
                match parse_packet::<DeckInfoRequest>("DeckInfo", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::deck::deck_info::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse DeckInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "FieldObjectInfo" => {
                match parse_packet::<FieldObjectInfoRequest>(
                    "FieldObjectInfo",
                    &request.request_data,
                ) {
                    Ok(req) => {
                        gameserver::logic::game::field::field_object_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse FieldObjectInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "PackInGameInfo" => {
                match parse_packet::<PackInGameInfoRequest>("PackInGameInfo", &request.request_data)
                {
                    Ok(req) => {
                        gameserver::logic::game::pack::pack_in_game_info::handle(&pool, uid, req)
                            .await
                    }
                    Err(e) => {
                        error!("Failed to parse PackInGameInfo: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            "LoginEvent" => {
                match parse_packet::<LoginEventRequest>("LoginEvent", &request.request_data) {
                    Ok(req) => {
                        gameserver::logic::game::login::login_event::handle(&pool, uid, req).await
                    }
                    Err(e) => {
                        error!("Failed to parse LoginEvent: {}", e);
                        GameResponse::error(400)
                    }
                }
            }
            // ... other routes
            _ => {
                warn!("Unknown batch method: {}", method_name);
                continue;
            }
        };

        responses.push(BatchResponseItem {
            path: request.path.clone(),
            response_data: game_response,
        });
    }

    Ok(HttpResponse::Ok().json(responses))
}

fn decrypt_batch_request(body: &str) -> Result<String, String> {
    let decrypted_bytes = AesTools::aes_decrypt_raw(body, crypto::aestools::DEFAULT_KEY)
        .map_err(|_| "AES decryption failed".to_string())?;
    String::from_utf8(decrypted_bytes).map_err(|e| format!("UTF-8 conversion failed: {}", e))
}

// Individual endpoint handlers
#[put("MissionInfo")]
pub async fn mission_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = match parse_packet::<MissionInfoRequest>("MissionInfo", &body) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to parse MissionInfo: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(400)));
        }
    };

    let response = gameserver::logic::game::mission::mission_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}

#[put("CharInfo")]
pub async fn char_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = match parse_packet::<CharInfoRequest>("CharInfo", &body) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to parse CharInfo: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(400)));
        }
    };

    let response = gameserver::logic::game::char::char_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}

#[put("RecipeInfo")]
pub async fn recipe_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = match parse_packet::<RecipeInfoRequest>("RecipeInfo", &body) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to parse RecipeInfo: {}", e);
            return Ok(HttpResponse::Ok().json(GameResponse::error(400)));
        }
    };

    let response = gameserver::logic::game::recipe::recipe_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}

// ... more handlers
