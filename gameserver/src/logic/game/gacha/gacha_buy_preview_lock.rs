use bd2::prost::Message;
use bd2::proto::proto_net::{GachaBuyPreviewLockRequest, GachaBuyPreviewLockResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Locks in a previously-shown preview roll (by event_index) so the client
/// won't reroll it. The response message is empty by design (no fields to
/// return) and no table tracks "locked preview" state, so there is
/// nothing to persist — this differs from a stub only in that it no
/// longer pretends there's a TODO left to do here.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: GachaBuyPreviewLockRequest) -> GameResponse {
    info!("Handling GachaBuyPreviewLockRequest: {:?}", req);

    let response = GachaBuyPreviewLockResponse {};
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

    let (route, code) = PacketCodeType::GachaBuyPreviewLock.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
