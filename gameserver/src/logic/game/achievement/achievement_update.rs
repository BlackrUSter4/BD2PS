use bd2::prost::Message;
use bd2::proto::proto_net::{AchievementUpdateRequest, AchievementUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::achievement::achievement_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-group progress accumulation.
pub async fn handle(pool: &SqlitePool, uid: i64, req: AchievementUpdateRequest) -> GameResponse {
    info!("Handling AchievementUpdateRequest: {:?}", req);

    if let (Some(group_id), Some(add_value)) = (req.group_id, req.add_value) {
        let contents_group = data::exceldb::get()
            .achievementtable
            .by_group(group_id)
            .next()
            .and_then(|a| a.contents_group);
        let _ = db::add_value(pool, uid, group_id, contents_group, add_value as i64).await;
    }

    let response = AchievementUpdateResponse {};

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

    let (route, code) = PacketCodeType::AchievementUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
