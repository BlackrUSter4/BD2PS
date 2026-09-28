use bd2::proto::proto_net::{BalanceVersionCheckRequest, BalanceVersionCheckResponse, Notify};
use crypto::network::GameResponse;
use tracing::info;
use bd2::prost::Message;
use common::packet_code::PacketCodeType;

pub async fn handle(req: BalanceVersionCheckRequest) -> GameResponse {
    info!("Handling balance version check request: {:?}", req);

    let response = BalanceVersionCheckResponse {};

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

    let (route, code) = PacketCodeType::BalanceVersionCheck.info();

    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}