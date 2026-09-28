use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PersonalInfoRequest, PersonalInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: PersonalInfoRequest) -> GameResponse {
    info!("Handling PersonalInfoRequest: {:?}", req);

    let response = PersonalInfoResponse {
        ..Default::default()
    };

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

    let (route, code) = PacketCodeType::PersonalInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
