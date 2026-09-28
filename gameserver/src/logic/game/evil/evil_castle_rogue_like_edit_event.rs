use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeEditEventRequest, EvilCastleRogueLikeEditEventResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_event_info;
use database::models::game::evil::evil_castle_rogue_like_event_info::EvilCastleRogueLikeEventInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Lets the client pick a specific alternate event (group_id/id) instead of
/// the auto-rolled one — real state, no reward math involved.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeEditEventRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeEditEventRequest: {:?}", req);

    let ev = EvilCastleRogueLikeEventInfo { index: 0, uid, group_id: req.group_id, id: req.id };
    let _ = evil_castle_rogue_like_event_info::upsert(pool, &ev).await;

    let response = EvilCastleRogueLikeEditEventResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeEditEvent.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
