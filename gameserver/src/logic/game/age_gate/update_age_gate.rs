use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, UpdateAgeGateRequest, UpdateAgeGateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::age_gate::age_gate_info as db;
use database::models::game::age_gate::age_gate_info::AgeGateInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account age-verification record.
pub async fn handle(pool: &SqlitePool, uid: i64, req: UpdateAgeGateRequest) -> GameResponse {
    info!("Handling UpdateAgeGateRequest: {:?}", req);

    let row = AgeGateInfo {
        uid,
        is_jp: req.is_jp.map(|b| b as i32),
        year: req.year,
        month: req.month,
        day: req.day,
    };
    let _ = db::save(pool, &row).await;

    let response = UpdateAgeGateResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::UpdateAgeGate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
