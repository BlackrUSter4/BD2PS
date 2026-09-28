use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeEnterRequest, EvilCastleRogueLikeEnterResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{
    evil_castle_rogue_like_floor_info, evil_castle_rogue_like_info, evil_castle_rogue_like_room_info,
    evil_castle_rogue_like_state_info,
};
use database::models::game::evil::evil_castle_rogue_like_info::EvilCastleRogueLikeInfo;
use database::models::game::evil::evil_castle_rogue_like_state_info::EvilCastleRogueLikeStateInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::roguelike;

/// Starts (or restarts) a roguelike run. Obsidian (the meta-currency,
/// growth levels) persists across runs; everything else resets — matches
/// how GiveUp explicitly hands back obsidian as "what you keep".
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeEnterRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeEnterRequest: {:?}", req);

    let level = req.level.unwrap_or(1);

    let prior_obsidian = evil_castle_rogue_like_info::get_one(pool, uid)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.obsidian)
        .unwrap_or(0);

    // Wipe the previous run's transient state.
    let _ = evil_castle_rogue_like_info::delete_evil_castle_rogue_like_info(pool, uid).await;
    let _ = evil_castle_rogue_like_state_info::delete_evil_castle_rogue_like_state_info(pool, uid).await;
    let _ = evil_castle_rogue_like_floor_info::delete_evil_castle_rogue_like_floor_info(pool, uid).await;
    let _ = evil_castle_rogue_like_room_info::delete_evil_castle_rogue_like_room_info(pool, uid).await;

    let default = exceldb::get().rldefaulttable.all().first();
    let start_gold = default.map(|d| d.entry_buy_price / 10).unwrap_or(60).max(60);

    let run = EvilCastleRogueLikeInfo {
        index: 0,
        uid,
        state_info_index: None,
        level: Some(level),
        floor_info_index: None,
        deck_info_index: None,
        char_info_index: None,
        costume_info_index: None,
        relic_info_index: None,
        choice_info_index: None,
        re_roll: Some(0),
        group_id: Some(0),
        id: Some(0),
        growth_info_index: None,
        event_info_index: None,
        shop_info_index: None,
        battle_level: Some(1),
        max_try_level: Some(0),
        obsidian: Some(prior_obsidian),
        rogue_like_gold: Some(start_gold),
        season: Some(1),
        regular_season: Some(1),
        season_reward: Some(0),
        max_reward_level: Some(0),
        highest_crystal_damage: Some(0),
    };
    let _ = evil_castle_rogue_like_info::add_evil_castle_rogue_like_info(pool, &run).await;

    let state = EvilCastleRogueLikeStateInfo {
        index: 0,
        uid,
        floor: Some(1),
        room: Some(0),
        state: Some(0),
    };
    let _ = evil_castle_rogue_like_state_info::add_evil_castle_rogue_like_state_info(pool, &state).await;

    roguelike::generate_floor(pool, uid, level, 1).await;
    let floor_info = roguelike::get_all_floors(pool, uid, 1).await;

    let response = EvilCastleRogueLikeEnterResponse {
        floor_info,
        choice_info: None,
        state_info: Some(roguelike::default_state(1, 0)),
        rogue_like_gold: Some(start_gold),
        re_roll: Some(0),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
