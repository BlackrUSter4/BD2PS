use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeEventReward, EvilCastleRogueLikeEventSelectRequest, EvilCastleRogueLikeEventSelectResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_state_info};
use sqlx::SqlitePool;
use tracing::info;

/// Picks one of a multi-choice event's options (choice_id). No table maps a
/// choice id to a specific reward, so this grants a small placeholder gold
/// amount — real state change (gold persists), placeholder magnitude.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeEventSelectRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeEventSelectRequest: {:?}", req);

    const PLACEHOLDER_GOLD: i32 = 50;
    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        run.rogue_like_gold = Some(run.rogue_like_gold.unwrap_or(0) + PLACEHOLDER_GOLD);
        let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
    }
    let _ = req.choice_id;

    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let response = EvilCastleRogueLikeEventSelectResponse {
        state_info: state.map(|s| super::roguelike::default_state(s.floor.unwrap_or(1), s.room.unwrap_or(0))),
        event_result: Some(1),
        clear_floor: None,
        event_reward_info: Some(EvilCastleRogueLikeEventReward {
            rogue_like_gold: Some(PLACEHOLDER_GOLD),
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
    let (route, code) = PacketCodeType::EvilCastleRogueLikeEventSelect.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
