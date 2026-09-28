use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, PackPreviewInfoRequest, PackPreviewInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Judgment call: this was previously two entirely fake hardcoded quest/title entries.
/// Quest/quest-title data belongs to the (separate, not-yet-audited) Quest system's own
/// tables — returning honestly empty here rather than fabricated placeholders.
/// `is_pack_event_reward` reflects whether a real PackJamEvent config exists at all.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: PackPreviewInfoRequest) -> GameResponse {
    info!("Handling PackPreviewInfoRequest: {:?}", req);

    let is_pack_event_reward = !data::exceldb::get().packjameventtable.is_empty();

    let response = PackPreviewInfoResponse {
        quest_info: vec![],
        quest_title_info: vec![],
        is_pack_event_reward: Some(is_pack_event_reward),
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

    let (route, code) = PacketCodeType::PackPreviewInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
