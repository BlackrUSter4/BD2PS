use bd2::prost::Message;
use bd2::proto::proto_net::{EventExchangeNextPageOpenRequest, EventExchangeNextPageOpenResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::event::event_exchange_info as ex_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account page advance, tracked in EventExchangeInfo.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventExchangeNextPageOpenRequest) -> GameResponse {
    info!("Handling EventExchangeNextPageOpenRequest: {:?}", req);

    let event_uid = req.event_uid.unwrap_or(0);
    let group_id = req.group_id.unwrap_or(0);

    let _ = ex_db::open_next_page(pool, uid, event_uid, group_id).await;

    let response = EventExchangeNextPageOpenResponse {
        // TODO: Fill in response fields
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EventExchangeNextPageOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
