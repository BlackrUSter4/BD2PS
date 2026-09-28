use bd2::prost::Message;
use bd2::proto::proto_net::{GuildSupporterDeleteRequest, GuildSupporterDeleteResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildSupporterDeleteRequest) -> GameResponse {
    info!("Handling GuildSupporterDeleteRequest: {:?}", req);

    if let Some(slot) = req.slot_index {
        let _ = sqlx::query("DELETE FROM GuildSupporterInfo WHERE Uid = ? AND SlotIndex = ?")
            .bind(uid)
            .bind(slot)
            .execute(pool)
            .await;
    }

    let response = GuildSupporterDeleteResponse {};
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
    let (route, code) = PacketCodeType::GuildSupporterDelete.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
