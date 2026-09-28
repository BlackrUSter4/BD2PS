use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, SkyWayDbInfo, SkyWayInfoRequest, SkyWayInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::sky::sky_way_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account sky-way progress.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SkyWayInfoRequest) -> GameResponse {
    info!("Handling SkyWayInfoRequest: {:?}", req);

    let sky_way_info = db::get_sky_way_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| SkyWayDbInfo { is_auto: r.is_auto, group_id: r.group_id, current_id: r.current_id, max_clear_level: r.max_clear_level })
        .collect();

    let response = SkyWayInfoResponse { sky_way_info };

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

    let (route, code) = PacketCodeType::SkyWayInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
