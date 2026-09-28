use bd2::prost::Message;
use bd2::proto::proto_net::{SupporterRemoveRequest, SupporterRemoveResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::supporter::supporter_slot_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterRemoveRequest) -> GameResponse {
    info!("Handling SupporterRemoveRequest: {:?}", req);

    if let Some(slot_index) = req.slot_index {
        let _ = db::delete_slot(pool, uid, slot_index).await;
    }

    let response = SupporterRemoveResponse {};
    
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
    
    let (route, code) = PacketCodeType::SupporterRemove.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}