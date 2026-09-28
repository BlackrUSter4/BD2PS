use bd2::prost::Message;
use bd2::proto::proto_net::{DispatchDbInfo, DispatchInfoRequest, DispatchInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::dispatch::dispatch_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account dispatch list (was silently returning an empty response with no stub
/// markers, despite a real, already-scaffolded DispatchInfo table sitting unused).
pub async fn handle(pool: &SqlitePool, uid: i64, req: DispatchInfoRequest) -> GameResponse {
    info!("Handling DispatchInfoRequest: {:?}", req);

    let dispatch_info = db::get_dispatch_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| DispatchDbInfo { id: r.id, server_now_time: r.server_now_time, end_time: r.end_time })
        .collect();

    let response = DispatchInfoResponse { dispatch_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::DispatchInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
