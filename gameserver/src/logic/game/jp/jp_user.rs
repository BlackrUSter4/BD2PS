use bd2::prost::Message;
use bd2::proto::proto_net::{JpUserRequest, JpUserResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::jp::jp_user_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real date-of-birth registration (Japan-region age-gate), encoded YYYYMMDD.
pub async fn handle(pool: &SqlitePool, uid: i64, req: JpUserRequest) -> GameResponse {
    info!("Handling JpUserRequest: {:?}", req);

    if let (Some(year), Some(month), Some(day)) = (req.year, req.month, req.day) {
        let encoded = year * 10000 + month * 100 + day;
        let _ = db::set_date_of_birth(pool, uid, encoded).await;
    }

    let response = JpUserResponse {};

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

    let (route, code) = PacketCodeType::JpUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
