use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, QuestImmortalResetRequest, QuestImmortalResetResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No "immortal quest" (no-death challenge) state is tracked anywhere in this project — no
/// column on UserQuest or any related table models it, and the response carries zero fields
/// to report back. Genuinely a no-op until that mechanic's state is captured/modeled.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: QuestImmortalResetRequest) -> GameResponse {
    info!("Handling QuestImmortalResetRequest: {:?}", req);

    let response = QuestImmortalResetResponse {};

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

    let (route, code) = PacketCodeType::QuestImmortalReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
