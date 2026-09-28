use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeBattleSkipRequest, EvilCastleRogueLikeBattleSkipResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_rogue_like_room_info, evil_castle_rogue_like_state_info};
use sqlx::SqlitePool;
use tracing::info;

/// Auto-resolves the current room's battle (no server-side battle sim
/// exists anywhere in this project — same "client simulates, server
/// trusts" pattern as Battle/Colosseum/Ib) — just marks the room cleared.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeBattleSkipRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeBattleSkipRequest: {:?}", req);

    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let (floor, room) = state.as_ref().map(|s| (s.floor.unwrap_or(1), s.room.unwrap_or(0))).unwrap_or((1, 0));
    let _ = evil_castle_rogue_like_room_info::set_clear(pool, uid, floor, room).await;

    let clear_room_info = evil_castle_rogue_like_room_info::get_by_floor(pool, uid, floor)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.number == Some(room))
        .map(|r| bd2::proto::proto_net::EvilCastleRogueLikeRoomInfo { number: r.number, group_id: r.group_id, id: r.id, is_clear: Some(1) });

    let response = EvilCastleRogueLikeBattleSkipResponse {
        state_info: Some(super::roguelike::default_state(floor, room)),
        choice_info: None,
        clear_floor: None,
        clear_room_info,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeBattleSkip.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
