use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardPresetDeleteRequest, IdCardPresetDeleteResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::id::id_card_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardPresetDeleteRequest) -> GameResponse {
    info!("Handling IdCardPresetDeleteRequest: {:?}", req);

    if let Some(id) = req.id {
        let _ = preset_db::delete_by_id(pool, uid, id).await;
    }

    let response = IdCardPresetDeleteResponse {};
    
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
    
    let (route, code) = PacketCodeType::IdCardPresetDelete.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}