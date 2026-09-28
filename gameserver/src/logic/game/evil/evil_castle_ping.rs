use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastlePingRequest, EvilCastlePingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_info;
use sqlx::SqlitePool;
use tracing::info;

/// A live heartbeat during a stage attempt — the request only carries
/// stage_index (no pack_id), so this just echoes the caller's best current
/// point across all their tower rows as a liveness signal.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastlePingRequest) -> GameResponse {
    info!("Handling EvilCastlePingRequest: {:?}", req);

    let point = evil_castle_info::get_evil_castle_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r.point)
        .max()
        .unwrap_or(0);

    let response = EvilCastlePingResponse { point: Some(point) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastlePing.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
