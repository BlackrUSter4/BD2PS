use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeEventChoiceRequest, EvilCastleRogueLikeEventChoiceResponse, EvilCastleRogueLikeEventReward, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_state_info};
use sqlx::SqlitePool;
use tracing::info;

/// Resolves an event choice against RLEventTable's real success-rate/effect
/// data. Effect ids reference RLEventEffectTypeTable, which this project has
/// no battle-effect interpreter for — so gold/heal/battle-level outcomes
/// implied by the effect are read where directly numeric, and everything
/// else is a documented no-op rather than a guess.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeEventChoiceRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeEventChoiceRequest: {:?}", req);

    let event_id = req.id.unwrap_or(1);
    let choice_idx = (req.r#type.unwrap_or(1) - 1).max(0) as usize;

    let mut success = true;
    let mut gold_delta = 0i32;

    if let Some(ev) = exceldb::get().rleventtable.get(event_id) {
        let rate = ev.event_success_rate.get(choice_idx).copied().unwrap_or(100);
        let mut seed = super::roguelike::new_seed(uid + event_id as i64);
        success = (super::roguelike::rand_u32(&mut seed) % 100) < rate.max(0) as u32;
        gold_delta = if success { 50 } else { -20 }; // placeholder magnitude — RLEventEffectTypeTable isn't interpreted
    }

    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        run.rogue_like_gold = Some((run.rogue_like_gold.unwrap_or(0) + gold_delta).max(0));
        let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
    }

    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let response = EvilCastleRogueLikeEventChoiceResponse {
        state_info: state.map(|s| super::roguelike::default_state(s.floor.unwrap_or(1), s.room.unwrap_or(0))),
        event_result: Some(if success { 1 } else { 0 }),
        clear_floor: None,
        event_reward_info: Some(EvilCastleRogueLikeEventReward {
            rogue_like_gold: Some(gold_delta),
            char_info: vec![],
            add_relic_info: vec![],
            remove_relic_info: vec![],
            choice_info: None,
            battle_level: None,
            heal_rate: None,
        }),
        clear_room_info: None,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeEventChoice.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
