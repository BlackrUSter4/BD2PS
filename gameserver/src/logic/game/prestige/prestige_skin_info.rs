use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PrestigeSkinDbInfo, PrestigeSkinInfoRequest, PrestigeSkinInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::prestige::prestige_skin_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real read (was silently returning an empty response with no stub markers despite a real,
/// already-scaffolded table sitting unused).
pub async fn handle(pool: &SqlitePool, uid: i64, req: PrestigeSkinInfoRequest) -> GameResponse {
    info!("Handling PrestigeSkinInfoRequest: {:?}", req);

    let prestige_skin_info = db::get_prestige_skin_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| PrestigeSkinDbInfo { costume_id: r.costume_id, costume_design_id: r.costume_design_id, is_set: r.is_set })
        .collect();

    let response = PrestigeSkinInfoResponse { prestige_skin_info };

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

    let (route, code) = PacketCodeType::PrestigeSkinInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
