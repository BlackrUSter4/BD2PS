use bd2::prost::Message;
use bd2::proto::proto_net::{SaveFieldCharControlDeckTypeRequest, SaveFieldCharControlDeckTypeResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::field_char_control_deck_type as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real persistence into the already-scaffolded FieldCharControlDeckType table.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SaveFieldCharControlDeckTypeRequest) -> GameResponse {
    info!("Handling SaveFieldCharControlDeckTypeRequest: {:?}", req);

    if let Some(value) = req.field_char_control_deck_type {
        let _ = db::set(pool, uid, value).await;
    }

    let response = SaveFieldCharControlDeckTypeResponse {};
    
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
    
    let (route, code) = PacketCodeType::SaveFieldCharControlDeckType.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}