use bd2::prost::Message;
use bd2::proto::proto_net::{GuildUpdateActionInfoRequest, GuildUpdateActionInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Marks the given action-log entry types as read (IsNotify = 0) for this account.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildUpdateActionInfoRequest) -> GameResponse {
    info!("Handling GuildUpdateActionInfoRequest: {:?}", req);

    for t in &req.r#type {
        let _ = sqlx::query("UPDATE GuildActionInfo SET IsNotify = 0 WHERE Uid = ? AND Type = ?")
            .bind(uid)
            .bind(serde_json::json!(t))
            .execute(pool)
            .await;
    }

    let response = GuildUpdateActionInfoResponse {};
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
    let (route, code) = PacketCodeType::GuildUpdateActionInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
