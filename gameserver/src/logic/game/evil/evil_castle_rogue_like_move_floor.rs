use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeMoveFloorRequest, EvilCastleRogueLikeMoveFloorResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_state_info};
use database::models::game::evil::evil_castle_rogue_like_state_info::EvilCastleRogueLikeStateInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::roguelike;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeMoveFloorRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeMoveFloorRequest: {:?}", req);

    let floor = req.floor.unwrap_or(1);
    let level = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten().and_then(|r| r.level).unwrap_or(1);

    roguelike::generate_floor(pool, uid, level, floor).await;

    let state = EvilCastleRogueLikeStateInfo { index: 0, uid, floor: Some(floor), room: Some(req.number.unwrap_or(0)), state: Some(0) };
    let _ = evil_castle_rogue_like_state_info::upsert(pool, &state).await;

    let response = EvilCastleRogueLikeMoveFloorResponse {
        state_info: Some(roguelike::default_state(floor, req.number.unwrap_or(0))),
        clear_floor: None,
        clear_room_info: None,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeMoveFloor.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
