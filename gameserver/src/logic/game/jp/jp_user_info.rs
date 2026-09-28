use bd2::prost::Message;
use bd2::proto::proto_net::{JpUserInfoRequest, JpUserInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::jp::jp_user_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real read of the Japan-region account fields.
pub async fn handle(pool: &SqlitePool, uid: i64, req: JpUserInfoRequest) -> GameResponse {
    info!("Handling JpUserInfoRequest: {:?}", req);

    let row = db::get_jp_user_info(pool, uid).await.unwrap_or_default().into_iter().next();

    let response = JpUserInfoResponse {
        accumulated_payment_amount: row.as_ref().and_then(|r| r.accumulated_payment_amount),
        date_of_birth: row.as_ref().and_then(|r| r.date_of_birth),
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

    let (route, code) = PacketCodeType::JpUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
